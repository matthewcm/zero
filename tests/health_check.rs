use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{PgConnection, Connection, PgPool, Executor};
use uuid::Uuid;
use std::net::TcpListener;

use zero::{configuration::{self, get_configuration, DatabaseSettings}, telemetry::{get_subscriber, init_subscriber}};

struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "test".into();
    let default_filter_level = "debug".into();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name, 
            default_filter_level, 
            std::io::sink
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            subscriber_name, 
            default_filter_level, 
            std::io::stdout
        );
        init_subscriber(subscriber);
    }
});

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
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
    let app = spawn_app().await;


    let client = reqwest::Client::new();
    let body = "name=Matthew%20Castrillon&email=mcm@matthewcm.dev";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");


    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "mcm@matthewcm.dev");
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form () {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let test_cases = vec![                  
        ("name=matthew", "missing the email"),
        ("email=matthew@gmail.com", "missing the name"),
        ("", "missing both email and mail")   ,
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");


        // Assert
        assert_eq!(400, response.status().as_u16(), "The API did not fail with 400 Bad Request :{}", error_message);

    }

}

async fn spawn_app() -> TestApp {

    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&configuration.database)
        .await;
        
    println!("Port: {}",port);
    let server = zero::startup::run( listener, db_pool.clone() )
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);
    // Created the server with random port. Now just need to find what time was used.

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(
        &config.connection_string_without_db().expose_secret()
    )
    .await
    .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create Database");

    let connection_pool = PgPool::connect(
        &config.connection_string().expose_secret()
    )
        .await
        .expect("Failed to connect to postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
        
}
