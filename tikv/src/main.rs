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
        plugin_api::registrar::PluginBuildInfo::get()
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

fn load_plugin() -> Result<plugin_api::registrar::Plugin, Box<dyn std::error::Error>> {
    let lib = Library::new("../tidb_query/target/debug/libtidb_query.so")?;
    // let lib = Library::new("../tidb_query/target/debug/libtidb_query.dylib")?;
    unsafe {
        let registrar: Symbol<plugin_api::registrar::PluginRegistrar> =
            lib.get(plugin_api::registrar::PLUGIN_REGISTRAR_SYMBOL)?;
        plugin_api::allocator::get_allocator();
        let plugin = registrar(plugin_api::allocator::get_allocator());
        std::mem::forget(lib);
        assert_eq!(
            plugin.plugin_build_info,
            plugin_api::registrar::PluginBuildInfo::get()
        );
        println!("Host: plugin loaded: {} {}", plugin.name, plugin.version);
        Ok(plugin)
    }
}
