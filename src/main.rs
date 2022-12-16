use std::net::TcpListener;
use zero::startup::run;

#[tokio::main]
async fn main() -> () {

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let server = run( listener ).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    println!("Server running on port: {} ", port)
}
