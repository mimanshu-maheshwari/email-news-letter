//! tests/health_check.rs 

// `tokio::test` is the testing equivalent of `tokio::main`. 
// It also spares you from having to specify the `#[test]` attribute
//
// You can inspect what code gets generated using 
// `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

fn spawn_app() -> String {
    let addr = "127.0.0.1:0";
    let listener = TcpListener::bind(addr).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = enl::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // we need to bring in `reqwest`
    // to perform HTTP requests against app 
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
