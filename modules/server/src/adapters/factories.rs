//! Repository implementations.

use std::time::SystemTime;

use async_trait::async_trait;
use serde::Deserialize;

use crate::application::repo::OrderRepository;
use crate::domain::entities::order::{NewOrder, Order};

use super::ServerResult;

#[allow(dead_code)]
#[derive(Clone, Deserialize)]
pub(crate) struct OrderFactory {}

#[async_trait(?Send)]
impl OrderRepository for OrderFactory {
    fn find_by_table_number(&self, number: i32) -> ServerResult<Order> {
        //self.order_repository.find_by_table_number(number).await?
        Ok(Order {
            id: 1,
            published_at: SystemTime::now(),
            table_number: number,
        }) // Dummy data until implemented.
    }
    fn create_order(&self, o: &NewOrder) -> ServerResult<Order> {
        Ok(Order {
            id: 1,
            published_at: SystemTime::now(),
            table_number: *o.table_number,
        }) // Dummy data until implemented.
    }
}
