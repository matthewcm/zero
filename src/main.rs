use std::net::TcpListener;

use actix_web::dev::Server;
use zero::run;

#[tokio::main]
async fn main() -> Result<((), ())> {

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    zero::run( listener ).expect("Failed to bind address")


}
