//! Repository implementations.

use diesel::prelude::*;
use serde::Deserialize;

use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};

use crate::application::features::get_all_active_customers;
use crate::application::repo::{CustomerRepository, ItemRepository, OrderRepository};
use crate::db_conn;
use crate::domain::entities::customer::{Customer, NewCustomer};
use crate::domain::entities::item::{Item, NewItem};
use crate::domain::entities::order::{NewOrder, Order};

use super::{db_connect, ServerError, ServerResult};

#[allow(dead_code)]
#[derive(Clone, Deserialize)]
pub(crate) struct OrderFactory {}

#[async_trait(?Send)]
impl OrderRepository for OrderFactory {
    fn find_by_table_number(&self, number: i32) -> ServerResult<Vec<Order>> {
        use crate::domain::entities::orders::dsl::*;
        let conn = db_conn!();
        let all_customers = get_all_active_customers(conn);
        if all_customers.is_err() {
            return Err(ServerError {
                error: "Unable to find customers!".to_string(),
            });
        }
        let order = Order::belonging_to(&all_customers.expect("Unable to find customers!"))
            .filter(table_number.eq(number))
            .select(Order::as_select())
            .load(conn);
        match order {
            Ok(order) => Ok(order),
            _ => Err(ServerError {
                error: "Unable to find item!".to_string(),
            }),
        }
    }

    fn find_specific_item(&self, number: i32, item: i32) -> ServerResult<Vec<Order>> {
        use crate::domain::entities::orders::dsl::*;
        let conn = db_conn!();
        let all_customers = get_all_active_customers(conn);
        if all_customers.is_err() {
            return Err(ServerError {
                error: "Unable to find customers!".to_string(),
            });
        }
        let order = Order::belonging_to(&all_customers.expect("Unable to find customers!"))
            .filter(table_number.eq(number).and(item_id.eq(item)))
            .select(Order::as_select())
            .load(conn);
        match order {
            Ok(order) => Ok(order),
            _ => Err(ServerError {
                error: "Unable to find item!".to_string(),
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

        let res = diesel::delete(orders.filter(item_id.eq(i))).execute(conn);

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

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Clone, Deserialize)]
pub(crate) struct CustomerFactory {}

#[async_trait(?Send)]
impl CustomerRepository for CustomerFactory {
    fn create(&self, n: &NewCustomer) -> ServerResult<Customer> {
        use crate::domain::entities::customers;
        let conn = db_conn!();

        let res = diesel::insert_into(customers::table)
            .values(n)
            .returning(Customer::as_returning())
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
