use actix_web::{HttpServer, App, web };
use actix_web::dev::Server;
use sqlx::PgConnection;

use crate::routes::{health_check, subscriptions};
 
pub fn run (
    listener:std::net::TcpListener,
    connection: PgConnection
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
