use std::pin::Pin;
use std::future::Future;

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

pub struct PluginRegistrar {
    pub name: &'static str,
    pub version: &'static str,
    pub endpoint_builder: fn() -> Box<dyn Endpoint>,
    pub plugin_build_info: PluginBuildInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginBuildInfo {
    pub api_version: &'static str,
    pub target: &'static str,
    pub host: &'static str,
    pub rustc: &'static str,
    pub target_arch: &'static str,
}

impl PluginBuildInfo {
    pub fn get() -> Self {
        Self {
            api_version: built_info::PKG_VERSION,
            target: built_info::TARGET,
            host: built_info::HOST,
            rustc: built_info::RUSTC_VERSION,
            target_arch: built_info::CFG_TARGET_ARCH,
        }
    }
}

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
