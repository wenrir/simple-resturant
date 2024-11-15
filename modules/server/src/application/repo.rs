use crate::domain::entities::order::{NewOrder, Order};
use anyhow::Result;
use async_trait::async_trait;

#[allow(dead_code)]
#[async_trait(?Send)]
pub(crate) trait OrderRepository {
    async fn find_by_table_number(&self, number: i32) -> Result<Order>;
    async fn create_order(&self, order: &NewOrder) -> Result<Order>;
    // Other traits goes here.
}

// This could ofcourse be moved to another files, as it's more an interface than a repo.
// But in terms of keeping it simple and memory allocations close together, I'm keeping it here.
#[allow(dead_code)]
#[async_trait(?Send)]
pub(crate) trait AbstractFeature<T> {
    async fn execute(&self) -> Result<T>;
}
