use futures::prelude::*;
use plugin_api::*;

declare_plugin!(TiDBQueryEndpoint::new());

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
