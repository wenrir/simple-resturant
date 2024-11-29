//! Repository implementations.

use diesel::prelude::*;
use serde::Deserialize;

use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};

use crate::application::features::{get_all_active_tables, get_table};
use crate::application::repo::{ItemRepository, OrderRepository, TableRepository};
use crate::db_conn;
use crate::domain::entities::item::{Item, NewItem};
use crate::domain::entities::order::{NewOrder, Order};
use crate::domain::entities::table::{NewTable, Table};

use super::{db_connect, ServerError, ServerResult};

#[derive(Clone, Deserialize)]
pub(crate) struct OrderFactory {}

#[async_trait(?Send)]
impl OrderRepository for OrderFactory {
    /// Calculate total for a table!
    fn total(&self, tid: &i32) -> ServerResult<i32> {
        let conn = db_conn!();
        let active_table = get_table(conn, tid);
        if active_table.is_err() {
            return Err(ServerError {
                error: "Unable to find tables!".to_string(),
            });
        }
        let _orders = Order::belonging_to(&active_table.expect("Unable to find tables!"))
            .select(Order::as_select())
            .load(conn)
            .expect("Unable to find table!");
        let mut total = 0;
        for order in _orders {
            let item = crate::domain::entities::items::dsl::items
                .filter(crate::domain::entities::items::dsl::id.eq(order.item_id))
                .select(Item::as_select())
                .first(conn)
                .expect("Unable to find item for order!");
            total += item.price * order.quantity
        }

        Ok(total)
    }
    /// Find an order
    fn find(&self, oid: &i32) -> ServerResult<Vec<Order>> {
        use crate::domain::entities::orders::dsl::*;
        let conn = db_conn!();
        let all_tables = get_all_active_tables(conn);
        if all_tables.is_err() {
            return Err(ServerError {
                error: "Unable to find tables!".to_string(),
            });
        }
        let order = Order::belonging_to(&all_tables.expect("Unable to find tables!"))
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

    /// Find orders for table
    fn find_table(&self, cid: &i32) -> ServerResult<Vec<Order>> {
        let conn = db_conn!();
        let table = get_table(conn, cid);
        if table.is_err() {
            return Err(ServerError {
                error: "Unable to find tables!".to_string(),
            });
        }
        let order = Order::belonging_to(&table.expect("Unable to find table!"))
            .select(Order::as_select())
            .load(conn);
        match order {
            Ok(order) => Ok(order),
            _ => Err(ServerError {
                error: "Unable to find order!".to_string(),
            }),
        }
    }

    /// Delete a tables order
    fn delete_table_order(&self, cid: &i32, oid: &i32) -> ServerResult<String> {
        use crate::domain::entities::orders::dsl::*;
        let conn = db_conn!();
        let table = get_table(conn, cid);
        if table.is_err() {
            return Err(ServerError {
                error: "Unable to find tables!".to_string(),
            });
        }
        let order = diesel::delete(
            Order::belonging_to(&table.expect("Unable to find table!")).filter(id.eq(oid)),
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

    /// Find all orders
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

    /// Create a new order
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

    /// Delete an order
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
    /// Create an item
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

    /// Get all items
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

    /// Get an item base on id
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
pub(crate) struct TableFactory {}

#[async_trait(?Send)]
impl TableRepository for TableFactory {
    /// Checkout a table by caclulating it's total
    fn checkout(&self, _id: &i32, _total: &i32) -> ServerResult<()> {
        use crate::domain::entities::tables::dsl::*;
        let conn = db_conn!();
        let r = diesel::update(tables)
            .filter(table_number.eq(_id).and(total.eq(-1_i32)))
            .set(total.eq(_total))
            .execute(conn);
        match r {
            Ok(_r) => Ok(()),
            Err(e) => {
                eprintln!("{:?}", e);
                Err(ServerError {
                    error: "Unable to create table!".to_string(),
                })
            }
        }
    }

    /// Create a table
    fn create(&self, n: &NewTable) -> ServerResult<Table> {
        use crate::domain::entities::tables;
        use crate::domain::entities::tables::dsl::*;
        let conn = db_conn!();

        let table = tables
            .select(Table::as_select())
            .filter(total.eq(-1_i32).and(table_number.eq(n.table_number)))
            .load(conn);
        if let Ok(c) = table {
            if !c.is_empty() {
                println!("CC >> {:?}", c);
                return Err(ServerError {
                    error: "Unable to checkin, table already occupied!".to_string(),
                });
            }
        }

        let res = diesel::insert_into(tables::table)
            .values(n)
            .returning(Table::as_returning())
            .get_result(conn);

        match res {
            Ok(r) => Ok(r),
            Err(e) => {
                eprintln!("{:?}", e);
                Err(ServerError {
                    error: "Unable to create table!".to_string(),
                })
            }
        }
    }

    /// Read all tables
    fn all(&self) -> ServerResult<Vec<Table>> {
        use crate::domain::entities::tables::dsl::*;
        let conn = db_conn!();
        let table = tables.select(Table::as_select()).load(conn).optional();
        match table {
            Ok(Some(c)) => Ok(c),
            Ok(None) => Ok(vec![]),
            _ => Err(ServerError {
                error: "Unable to find table!".to_string(),
            }),
        }
    }

    /// Get specific table..
    fn get(&self, _id: &i32) -> ServerResult<Table> {
        use crate::domain::entities::tables::dsl::*;
        let conn = db_conn!();
        let table = tables
            .filter(table_number.eq(_id).and(total.eq(-1_i32)))
            .select(Table::as_select())
            .first(conn);
        match table {
            Ok(table) => Ok(table),
            _ => Err(ServerError {
                error: "Unable to find item!".to_string(),
            }),
        }
    }
}
