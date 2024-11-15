//! application/features.rs
//! Application features (usecases)

use anyhow::Result;
use async_trait::async_trait;

use super::repo::{AbstractFeature, OrderRepository};
use crate::domain::entities::order::{NewOrder, Order};

#[allow(dead_code)]
pub(crate) struct OrderFeature<'a> {
    table_number: &'a i32,
    order_repository: &'a dyn OrderRepository,
}

#[allow(dead_code)]
impl<'a> OrderFeature<'a> {
    pub(crate) fn new(table: &'a i32, repository: &'a dyn OrderRepository) -> Self {
        OrderFeature {
            table_number: table,
            order_repository: repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractFeature<Order> for OrderFeature<'a> {
    async fn execute(&self) -> Result<Order> {
        let order = self
            .order_repository
            .create_order(&NewOrder {
                table_number: self.table_number,
            })
            .await?;
        Ok(order)
    }
}
