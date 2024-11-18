use crate::{
    adapters::ServerResult, // Todo, move me out of adapter.
    domain::entities::order::{NewOrder, Order},
};
use async_trait::async_trait;

#[allow(dead_code)]
#[async_trait(?Send)]
pub(crate) trait OrderRepository {
    fn find_by_table_number(&self, number: i32) -> ServerResult<Order>;
    fn create_order(&self, order: &NewOrder) -> ServerResult<Order>;
    // Other traits goes here.
}

// This could ofcourse be moved to another files, as it's more an interface than a repo.
// But in terms of keeping it simple and memory allocations close together, I'm keeping it here.
#[allow(dead_code)]
#[async_trait(?Send)]
pub(crate) trait AbstractFeature<T> {
    async fn execute(&self) -> ServerResult<T>;
}
