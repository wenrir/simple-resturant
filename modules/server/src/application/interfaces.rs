use crate::domain::error::ApiError;
use async_trait::async_trait;

#[async_trait]
pub trait AbstractUseCase<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}
