mod plugin_manager;

use std::time::Duration;

use futures::prelude::*;
use plugin_api::*;
use semver::VersionReq;

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
        plugin_api::registrar::BuildInfo::get()
    );

    let plugin_manager = plugin_manager::PluginManager::start("../tidb_query/target/debug");

    let task_factory = || async {
        let req = "k1".to_string().into_bytes();
        println!("Host: handle new request: [Proto] Get Key {:?}", &req);
        let resp = plugin_manager
            .handle_request(
                "tidb_query",
                &VersionReq::parse("*").unwrap(),
                req,
                Box::new(StoreImpl),
            )
            .unwrap()
            .await
            .unwrap();
        println!(
            "Host: coprocessor response: {:?}",
            String::from_utf8(resp).unwrap()
        );
    };

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        loop {
            task_factory().await;
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    Ok(())
}
