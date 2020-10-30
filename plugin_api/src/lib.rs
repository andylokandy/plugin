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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plugin {
    pub name: &'static str,
    pub version: &'static str,
    pub plugin_build_info: PluginBuildInfo,
    pub endpoint_builder: fn() -> Box<dyn Endpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginBuildInfo {
    pub api_version: &'static str,
    pub target: &'static str,
    pub host: &'static str,
    pub rustc: &'static str,
}

impl PluginBuildInfo {
    pub const fn get() -> Self {
        Self {
            api_version: env!("API_VERSION"),
            target: env!("TARGET"),
            host: env!("HOST"),
            rustc: env!("RUSTC_VERSION"),
        }
    }
}
