//! Repository implementations.

use diesel::prelude::*;
use serde::Deserialize;

use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};

use crate::application::features::{get_all_active_customers, get_customer};
use crate::application::repo::{CustomerRepository, ItemRepository, OrderRepository};
use crate::db_conn;
use crate::domain::entities::customer::{Customer, NewCustomer};
use crate::domain::entities::item::{Item, NewItem};
use crate::domain::entities::order::{NewOrder, Order};

use super::{db_connect, ServerError, ServerResult};

#[derive(Clone, Deserialize)]
pub(crate) struct OrderFactory {}

#[async_trait(?Send)]
impl OrderRepository for OrderFactory {
    fn find(&self, oid: &i32) -> ServerResult<Vec<Order>> {
        use crate::domain::entities::orders::dsl::*;
        let conn = db_conn!();
        let all_customers = get_all_active_customers(conn);
        if all_customers.is_err() {
            return Err(ServerError {
                error: "Unable to find customers!".to_string(),
            });
        }
        let order = Order::belonging_to(&all_customers.expect("Unable to find customers!"))
            .filter(id.eq(oid))
            .select(Order::as_select())
            .load(conn);
        match order {
            Ok(order) => Ok(order),
            _ => Err(ServerError {
                error: "Unable to find order!".to_string(),
            }),
        }
    }

    fn find_customer(&self, cid: &i32) -> ServerResult<Vec<Order>> {
        let conn = db_conn!();
        let customer = get_customer(conn, cid);
        if customer.is_err() {
            return Err(ServerError {
                error: "Unable to find customers!".to_string(),
            });
        }
        let order = Order::belonging_to(&customer.expect("Unable to find customer!"))
            .select(Order::as_select())
            .load(conn);
        match order {
            Ok(order) => Ok(order),
            _ => Err(ServerError {
                error: "Unable to find order!".to_string(),
            }),
        }
    }

    fn delete_customer_order(&self, cid: &i32, oid: &i32) -> ServerResult<String> {
        use crate::domain::entities::orders::dsl::*;
        let conn = db_conn!();
        let customer = get_customer(conn, cid);
        if customer.is_err() {
            return Err(ServerError {
                error: "Unable to find customers!".to_string(),
            });
        }
        let order = diesel::delete(
            Order::belonging_to(&customer.expect("Unable to find customer!")).filter(id.eq(oid)),
        )
        .execute(conn);
        match order {
            Ok(r) => {
                if r == 0 {
                    return Err(ServerError {
                        error: "Unable to find order id!".to_string(),
                    });
                }
                Ok("OK".to_string())
            }
            _ => Err(ServerError {
                error: "Unable to find order!".to_string(),
            }),
        }
    }

    fn all(&self) -> ServerResult<Vec<Order>> {
        use crate::domain::entities::orders::dsl::*;
        let conn = db_conn!();
        let order = orders.select(Order::as_select()).load(conn).optional();
        match order {
            Ok(Some(order)) => Ok(order),
            Ok(None) => Ok(vec![]),
            _ => Err(ServerError {
                error: "Unable to find orders!".to_string(),
            }),
        }
    }

    fn create(&self, o: &NewOrder) -> ServerResult<Order> {
        use crate::domain::entities::orders;
        let conn = db_conn!();

        let res = diesel::insert_into(orders::table)
            .values(o)
            .returning(Order::as_returning())
            .get_result(conn);

        match res {
            Ok(r) => Ok(r),
            Err(e) => {
                eprintln!("{:?}", e);
                Err(ServerError {
                    error: "Unable to create order!".to_string(),
                })
            }
        }
    }
    fn delete(&self, i: &i32) -> ServerResult<()> {
        use crate::domain::entities::orders::dsl::*;
        let conn = db_conn!();

        let res = diesel::delete(orders.filter(id.eq(i))).execute(conn);

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("{:?}", e);
                Err(ServerError {
                    error: "Unable to create order!".to_string(),
                })
            }
        }
    }
}

#[derive(Clone, Deserialize)]
pub(crate) struct ItemFactory {}

#[async_trait(?Send)]
impl ItemRepository for ItemFactory {
    fn create(&self, n: &NewItem) -> ServerResult<Item> {
        use crate::domain::entities::items;
        let conn = db_conn!();

        let res = diesel::insert_into(items::table)
            .values(n)
            .returning(Item::as_returning())
            .get_result(conn);

        match res {
            Ok(r) => Ok(r),
            Err(e) => {
                eprintln!("{:?}", e);
                Err(ServerError {
                    error: "Unable to create item!".to_string(),
                })
            }
        }
    }

    fn all(&self) -> ServerResult<Vec<Item>> {
        use crate::domain::entities::items::dsl::*;
        let conn = db_conn!();
        let item = items.select(Item::as_select()).load(conn).optional();
        match item {
            Ok(Some(item)) => Ok(item),
            Ok(None) => Ok(vec![]),
            _ => Err(ServerError {
                error: "Unable to find item!".to_string(),
            }),
        }
    }

    fn get(&self, _id: &i32) -> ServerResult<Item> {
        use crate::domain::entities::items::dsl::*;
        let conn = db_conn!();
        let item = items
            .filter(id.eq(_id))
            .select(Item::as_select())
            .first(conn);
        match item {
            Ok(item) => Ok(item),
            _ => Err(ServerError {
                error: "Unable to find item!".to_string(),
            }),
        }
    }
}

#[derive(Clone, Deserialize)]
pub(crate) struct CustomerFactory {}

#[async_trait(?Send)]
impl CustomerRepository for CustomerFactory {
    fn create(&self, n: &NewCustomer) -> ServerResult<Customer> {
        use crate::domain::entities::customers;
        use crate::domain::entities::customers::dsl::*;
        let conn = db_conn!();

        let customer = customers
            .select(Customer::as_select())
            .filter(total.eq(0).and(table_number.eq(n.table_number)))
            .load(conn);
        if let Ok(c) = customer {
            if !c.is_empty() {
                return Err(ServerError {
                    error: "Unable to checkin, table already occupied!".to_string(),
                });
            }
        }

        let res = diesel::insert_into(customers::table)
            .values(n)
            .returning(Customer::as_returning())
            .get_result(conn);

        match res {
            Ok(r) => Ok(r),
            Err(e) => {
                eprintln!("{:?}", e);
                Err(ServerError {
                    error: "Unable to create customer!".to_string(),
                })
            }
        }
    }

    fn all(&self) -> ServerResult<Vec<Customer>> {
        use crate::domain::entities::customers::dsl::*;
        let conn = db_conn!();
        let customer = customers
            .select(Customer::as_select())
            .load(conn)
            .optional();
        match customer {
            Ok(Some(c)) => Ok(c),
            Ok(None) => Ok(vec![]),
            _ => Err(ServerError {
                error: "Unable to find customer!".to_string(),
            }),
        }
    }
    fn get(&self, _id: &i32) -> ServerResult<Customer> {
        use crate::domain::entities::customers::dsl::*;
        let conn = db_conn!();
        let customer = customers
            .filter(id.eq(_id))
            .select(Customer::as_select())
            .first(conn);
        match customer {
            Ok(customer) => Ok(customer),
            _ => Err(ServerError {
                error: "Unable to find item!".to_string(),
            }),
        }
    }
}
