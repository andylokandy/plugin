// use plugin_api::{PluginRegistar, Callback};
use futures::prelude::*;
use libloading::{Library, Symbol};
use plugin_api::*;

use std::time::Duration;

struct StoreImpl;

impl Store for StoreImpl {
    fn get(&self, key: Key) -> BoxFuture<Value> {
        async {
            println!("Host: scaning key {:?}", &key);
            tokio::time::sleep(Duration::from_millis(500)).await;
            let mut val = "[val] of ".to_string().into_bytes();
            println!("Host: returning val of key {:?}", &key);
            val.extend(key);
            val
        }
        .boxed()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Host: host plugin system info: {:#?}",
        PluginBuildInfo::get()
    );

    let plugin = load_plugin()?;
    let endpoint = (plugin.endpoint_builder)();

    let task = async {
        let req = "k1".to_string().into_bytes();
        println!("Host: handle new request: [Proto] Get Key {:?}", &req);
        let resp = endpoint
            .handle_request(req, Box::new(StoreImpl))
            .await
            .unwrap();
        println!(
            "Host: coprocessor responce: {:?}",
            String::from_utf8(resp).unwrap()
        );
    };

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(task);

    Ok(())
}

fn load_plugin() -> Result<PluginRegistrar, Box<dyn std::error::Error>> {
    let lib = Library::new("../tidb_query/target/debug/libtidb_query.dylib")?;
    unsafe {
        let register_fn: Symbol<fn() -> PluginRegistrar> = lib.get(b"register")?;
        let registrar = register_fn();
        assert_eq!(registrar.plugin_build_info, PluginBuildInfo::get());
        println!(
            "Host: plugin loaded: {} {}",
            registrar.name, registrar.version
        );
        Ok(registrar)
    }
}
