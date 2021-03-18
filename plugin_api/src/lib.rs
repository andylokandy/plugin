pub mod allocator;
pub mod registrar;

use std::future::Future;
use std::pin::Pin;

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

pub trait Store: Send + Sync {
    fn get(&self, key: Key) -> BoxFuture<Value>;
}

pub trait Endpoint {
    fn handle_request(&self, req: Vec<u8>, store: Box<dyn Store>)
        -> BoxFuture<Result<Vec<u8>, ()>>;
}
