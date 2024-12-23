//! Repository implementations.

use diesel::prelude::*;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;

use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};

use crate::application::features::{get_all_active_tables, get_table};
use crate::application::repo::{ItemRepository, OrderRepository, TableRepository};
use crate::db_conn;
use crate::domain::entities::item::{Item, NewItem};
use crate::domain::entities::order::{NewOrder, Order};
use crate::domain::entities::table::{NewTable, Table};

use super::{ServerError, ServerResult};
use log::error;

/// Macro database query with ServerError handling.
macro_rules! db_query {
    ($query:expr, $error:expr) => {
        match $query {
            Ok(result) => Ok(result),
            Err(err) => {
                error!("{:?}", err);
                Err(ServerError {
                    error: $error.to_string(),
                })
            }
        }
    };
}

/// Macro database query (optional) with ServerError handling.
macro_rules! db_query_optional {
    ($query:expr, $error:expr, $rnone:expr) => {
        match $query {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Ok($rnone),
            Err(err) => {
                error!("{:?}", err);
                Err(ServerError {
                    error: $error.to_string(),
                })
            }
        }
    };
}

#[derive(Clone, Debug)]
pub(crate) struct OrderFactory {
    pub(crate) connection_pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait(?Send)]
impl OrderRepository for OrderFactory {
    /// Calculate total for a table!
    fn total(&self, tid: &i32) -> ServerResult<i32> {
        let active_table = get_table(db_conn!(self), tid);
        if active_table.is_err() {
            return Err(ServerError {
                error: "Unable to find tables!".to_string(),
            });
        }
        let _orders = Order::belonging_to(&active_table.expect("Unable to find tables!"))
            .select(Order::as_select())
            .load(db_conn!(self))
            .expect("Unable to find table!");
        let mut total = 0;
        for order in _orders {
            let item = crate::domain::entities::items::dsl::items
                .filter(crate::domain::entities::items::dsl::id.eq(order.item_id))
                .select(Item::as_select())
                .first(db_conn!(self))
                .expect("Unable to find item for order!");
            total += item.price * order.quantity
        }

        Ok(total)
    }
    /// Find an order
    fn find(&self, oid: &i32) -> ServerResult<Vec<Order>> {
        use crate::domain::entities::orders::dsl::*;
        let all_tables = get_all_active_tables(db_conn!(self));
        if all_tables.is_err() {
            return Err(ServerError {
                error: "Unable to find tables!".to_string(),
            });
        }
        db_query!(
            Order::belonging_to(&all_tables.expect("Unable to find tables!"))
                .filter(id.eq(oid))
                .select(Order::as_select())
                .load(db_conn!(self)),
            "Unable to find order!"
        )
    }

    /// Find orders for table
    fn find_table(&self, cid: &i32) -> ServerResult<Vec<Order>> {
        let table = get_table(db_conn!(self), cid);
        if table.is_err() {
            return Err(ServerError {
                error: "Unable to find tables!".to_string(),
            });
        }
        db_query!(
            Order::belonging_to(&table.expect("Unable to find table!"))
                .select(Order::as_select())
                .load(db_conn!(self)),
            "Unable to find order for table"
        )
    }

    /// Delete a tables order
    fn delete_table_order(&self, cid: &i32, oid: &i32) -> ServerResult<String> {
        use crate::domain::entities::orders::dsl::*;
        let table = get_table(db_conn!(self), cid);
        if table.is_err() {
            return Err(ServerError {
                error: "Unable to find tables!".to_string(),
            });
        }
        let order = diesel::delete(
            Order::belonging_to(&table.expect("Unable to find table!")).filter(id.eq(oid)),
        )
        .execute(db_conn!(self));
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
        db_query_optional!(
            orders
                .select(Order::as_select())
                .load(db_conn!(self))
                .optional(),
            "Unable to find orders",
            vec![]
        )
    }

    /// Create a new order
    fn create(&self, o: &NewOrder) -> ServerResult<Order> {
        use crate::domain::entities::orders;
        db_query!(
            diesel::insert_into(orders::table)
                .values(o)
                .returning(Order::as_returning())
                .get_result(db_conn!(self)),
            "Unable to create order!"
        )
    }

    /// Delete an order
    fn delete(&self, i: &i32) -> ServerResult<()> {
        use crate::domain::entities::orders::dsl::*;
        let res = diesel::delete(orders.filter(id.eq(i))).execute(db_conn!(self));
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
                Err(ServerError {
                    error: "Unable to create order!".to_string(),
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ItemFactory {
    pub(crate) connection_pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait(?Send)]
impl ItemRepository for ItemFactory {
    /// Create an item
    fn create(&self, n: &NewItem) -> ServerResult<Item> {
        use crate::domain::entities::items;
        db_query!(
            diesel::insert_into(items::table)
                .values(n)
                .returning(Item::as_returning())
                .get_result(db_conn!(self)),
            "Unable to create item"
        )
    }

    /// Get all items
    fn all(&self) -> ServerResult<Vec<Item>> {
        use crate::domain::entities::items::dsl::*;
        db_query_optional!(
            items
                .select(Item::as_select())
                .load(db_conn!(self))
                .optional(),
            "Unable to find all items ",
            vec![]
        )
    }

    /// Get an item base on id
    fn get(&self, _id: &i32) -> ServerResult<Item> {
        use crate::domain::entities::items::dsl::*;
        db_query!(
            items
                .filter(id.eq(_id))
                .select(Item::as_select())
                .first(db_conn!(self)),
            "Unable to get item"
        )
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TableFactory {
    pub(crate) connection_pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait(?Send)]
impl TableRepository for TableFactory {
    /// Checkout a table by caclulating it's total
    fn checkout(&self, _id: &i32, _total: &i32) -> ServerResult<()> {
        use crate::domain::entities::tables::dsl::*;
        let r = diesel::update(tables)
            .filter(table_number.eq(_id).and(total.eq(-1_i32)))
            .set(total.eq(_total))
            .execute(db_conn!(self));
        match r {
            Ok(_r) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
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

        let table = tables
            .select(Table::as_select())
            .filter(total.eq(-1_i32).and(table_number.eq(n.table_number)))
            .load(db_conn!(self));
        if let Ok(c) = table {
            if !c.is_empty() {
                return Err(ServerError {
                    error: "Unable to checkin, table already occupied!".to_string(),
                });
            }
        }

        db_query!(
            diesel::insert_into(tables::table)
                .values(n)
                .returning(Table::as_returning())
                .get_result(db_conn!(self)),
            "Unable to create table"
        )
    }

    /// Read all tables
    fn all(&self) -> ServerResult<Vec<Table>> {
        use crate::domain::entities::tables::dsl::*;
        db_query_optional!(
            tables
                .select(Table::as_select())
                .load(db_conn!(self))
                .optional(),
            "Unable to find all tables!",
            vec![]
        )
    }

    /// Get specific table..
    fn get(&self, _id: &i32) -> ServerResult<Table> {
        use crate::domain::entities::tables::dsl::*;
        db_query!(
            tables
                .filter(table_number.eq(_id).and(total.eq(-1_i32)))
                .select(Table::as_select())
                .first(db_conn!(self)),
            "Unable to find specific table "
        )
    }
}
