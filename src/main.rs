use std::net::TcpListener;
use sqlx::{Connection, PgPool};
use zero::{startup::run, configuration::get_configuration};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to read configuration");
    
    let connection = PgPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failed to connect to postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}
