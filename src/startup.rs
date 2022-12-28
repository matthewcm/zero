use actix_web::middleware::Logger;
use actix_web::{HttpServer, App, web };
use actix_web::dev::Server;
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};
 
pub fn run (
    listener:std::net::TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
