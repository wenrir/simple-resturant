use crate::{
    adapters::ServerResult, // Todo, move me out of adapter.
    domain::entities::{
        customer::{Customer, NewCustomer},
        item::{Item, NewItem},
        order::{NewOrder, Order},
    },
};
use async_trait::async_trait;

#[async_trait(?Send)]
pub(crate) trait OrderRepository {
    fn find_by_table_number(&self, number: i32) -> ServerResult<Vec<Order>>;
    fn find_specific_item(&self, number: i32, item: i32) -> ServerResult<Vec<Order>>;
    fn create(&self, order: &NewOrder) -> ServerResult<Order>;
    fn delete(&self, item_id: &i32) -> ServerResult<()>;
    // Other traits goes here.
}

#[async_trait(?Send)]
pub(crate) trait ItemRepository {
    fn create(&self, item: &NewItem) -> ServerResult<Item>;
    fn get(&self, id: &i32) -> ServerResult<Item>;
    // Other traits goes here.
}

#[async_trait(?Send)]
pub(crate) trait CustomerRepository {
    fn create(&self, item: &NewCustomer) -> ServerResult<Customer>;
    fn get(&self, id: &i32) -> ServerResult<Customer>;
    // Other traits goes here.
}

// This could ofcourse be moved to another files, as it's more an interface than a repo.
// But in terms of keeping it simple and memory allocations close together, I'm keeping it here.
#[allow(dead_code)]
#[async_trait(?Send)]
pub(crate) trait AbstractFeature<T> {
    async fn execute(&self) -> ServerResult<T>;
}
