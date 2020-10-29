use plugin_api::*;
use futures::prelude::*;

#[no_mangle]
pub fn register() -> PluginRegistrar {
    PluginRegistrar {
        name: "tidb_query",
        version: "0.0.0",
        endpoint_builder,
        plugin_build_info: PluginBuildInfo::get(),
    }
}

fn endpoint_builder() -> Box<dyn Endpoint> {
    Box::new(TiDBQueryEndpoint::new())
}

struct TiDBQueryEndpoint {}

impl TiDBQueryEndpoint {
    pub fn new() -> TiDBQueryEndpoint {
        TiDBQueryEndpoint {}
    }
}

impl Endpoint for TiDBQueryEndpoint {
    fn handle_request(
        &self,
        req: Vec<u8>,
        store: Box<dyn Store>,
    ) -> BoxFuture<Result<Vec<u8>, ()>> {
        async move {
            println!("Plugin: start to get key {:?}", &req);
            let v = store.get("k1".to_string().into_bytes()).await;
            println!("Plugin: got val: {:?}", v);
            let resp = format!("[Proto] val: {:?}", v).into_bytes();
            Ok(resp)
        }
        .boxed()
    }
}
