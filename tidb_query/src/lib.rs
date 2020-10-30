use futures::prelude::*;
use plugin_api::*;

#[no_mangle]
pub fn register() -> Plugin {
    Plugin {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        plugin_build_info: PluginBuildInfo::get(),
        endpoint_builder,
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
