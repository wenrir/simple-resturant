use crate::{
    adapters::ServerResult, // Todo, move me out of adapter.
    domain::entities::{
        item::{Item, NewItem},
        order::{NewOrder, Order},
        table::{NewTable, Table},
    },
};
use async_trait::async_trait;

#[async_trait(?Send)]
pub(crate) trait OrderRepository {
    fn find(&self, id: &i32) -> ServerResult<Vec<Order>>;
    fn find_table(&self, id: &i32) -> ServerResult<Vec<Order>>;
    fn delete_table_order(&self, cid: &i32, oid: &i32) -> ServerResult<String>;
    fn create(&self, order: &NewOrder) -> ServerResult<Order>;
    fn delete(&self, item_id: &i32) -> ServerResult<()>;
    fn all(&self) -> ServerResult<Vec<Order>>;
    fn total(&self, oid: &i32) -> ServerResult<i32>;
}

#[async_trait(?Send)]
pub(crate) trait ItemRepository {
    fn create(&self, item: &NewItem) -> ServerResult<Item>;
    fn get(&self, id: &i32) -> ServerResult<Item>;
    fn all(&self) -> ServerResult<Vec<Item>>;
}

#[async_trait(?Send)]
pub(crate) trait TableRepository {
    fn create(&self, item: &NewTable) -> ServerResult<Table>;
    fn get(&self, id: &i32) -> ServerResult<Table>;
    fn checkout(&self, id: &i32, total: &i32) -> ServerResult<()>;
    fn all(&self) -> ServerResult<Vec<Table>>;
}

// This could ofcourse be moved to another files, as it's more an interface than a repo.
// But in terms of keeping it simple and memory allocations close together, I'm keeping it here.
#[allow(dead_code)]
#[async_trait(?Send)]
pub(crate) trait AbstractFeature<T> {
    async fn execute(&self) -> ServerResult<T>;
}
