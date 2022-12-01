use actix_web::{HttpRequest, HttpResponse, HttpServer, App, web, Responder, dev::Server};

async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener:std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
