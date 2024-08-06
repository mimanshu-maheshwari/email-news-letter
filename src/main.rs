use std::net::TcpListener;

use enl::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr).expect("Failed to bind address to tcp listener");
    println!("{:#?}", listener);
    run(listener)?.await
}
