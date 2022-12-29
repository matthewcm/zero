use secrecy::ExposeSecret;
use zero::telemetry::init_subscriber;
use std::net::TcpListener;
use sqlx::PgPool;

use zero::{startup::run, configuration::get_configuration, telemetry::get_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string().expose_secret()
    )
    .await
    .expect("Failed to connect to postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
