//! tests/health_check.rs 

// `tokio::test` is the testing equivalent of `tokio::main`. 
// It also spares you from having to specify the `#[test]` attribute
//
// You can inspect what code gets generated using 
// `cargo expand --test health_check` (<- name of the test file)

fn spawn_app() {
    let server = enl::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // we need to bring in `reqwest`
    // to perform HTTP requests against app 
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
