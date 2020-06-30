mod handlers;
use actix_web::{web, App, HttpServer};
use handlers::{echo, hello};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/temp", web::get().to(hello::temperature))
            .route("/temp/{temp}", web::get().to(hello::temperature))
            .route("/echo", web::get().to(echo::echo_request))
            .route("/", web::get().to(hello::greet))
            .route("/{name}", web::get().to(hello::greet))
    })
    .keep_alive(30)
    .bind("0.0.0.0:8000")?
    .run()
    .await
}