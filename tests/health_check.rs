use sqlx::{PgConnection, Connection};
use std::net::TcpListener;

use zero::configuration::{self, get_configuration};



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

#[tokio::test]
async fn subscribe_returns_200_for_valid_form () {
    // Arrange
    let app_address = spawn_app();

    let configuration = get_configuration().expect("Failed to read configuration");

    let connection_string = configuration.database.connection_string();

    let connection = PgConnection::connect(
        &connection_string
    ).await
    .expect("Failed to connect to postgres.");

    let client = reqwest::Client::new();
    let body = "name=Matthew%20Castrillon&email=mcm@matthewcm.dev";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");


    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form () {
    // Arrange
    let app_address = spawn_app();

    let client = reqwest::Client::new();
    let test_cases = vec![                  
        ("name=matthew", "missing the email"),
        ("email=matthew@gmail.com", "missing the name"),
        ("", "missing both email and mail")   ,
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");


        // Assert
        assert_eq!(400, response.status().as_u16(), "The API did not fail with 400 Bad Request :{}", error_message);

    }

}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    println!("{}",port);
    let server = zero::startup::run( listener )
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);
    // Created the server with random port. Now just need to find what time was used.

    format!("http://127.0.0.1:{}", port)
}
