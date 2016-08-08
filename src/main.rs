#[macro_use]
extern crate tarpc;

mod hello_service {
    service! {
        rpc hello(name: String) -> String;
    }
}

use hello_service::Service as HelloService;

struct HelloServer;

impl HelloService for HelloServer {
    fn hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }
}

fn main() {
    let addr = "localhost:10000";
    let server_handle = HelloServer.spawn(addr).unwrap();
    let client = hello_service::Client::new(addr).unwrap();
    assert_eq!("Hello, Mom!", client.hello("Mom".into()).unwrap());
    drop(client);
    server_handle.shutdown();
}
