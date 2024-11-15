//! Repository implementations.

use std::time::SystemTime;

use async_trait::async_trait;
use diesel::PgConnection;

use crate::application::repo::OrderRepository;
use crate::domain::entities::order::{NewOrder, Order};
use anyhow::Result;

#[allow(dead_code)]
pub(crate) struct OrderFactory {
    db_connection: PgConnection,
}

#[async_trait(?Send)]
impl OrderRepository for OrderFactory {
    async fn find_by_table_number(&self, number: i32) -> Result<Order> {
        //self.order_repository.find_by_table_number(number).await?
        Ok(Order {
            id: 1,
            published_at: SystemTime::now(),
            table_number: number,
        }) // Dummy data until implemented.
    }
    async fn create_order(&self, o: &NewOrder) -> Result<Order> {
        Ok(Order {
            id: 1,
            published_at: SystemTime::now(),
            table_number: *o.table_number,
        }) // Dummy data until implemented.
    }
}
