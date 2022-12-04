use std::{net::TcpListener };



#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app_address = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");


    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    println!("{}",port);
    let server = zero::run( listener )
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);
    // Created the server with random port. Now just need to find what time was used.
    
    format!("http://127.0.0.1:{}", port)
}
