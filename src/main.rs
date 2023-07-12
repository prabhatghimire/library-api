use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn helo_world() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(helo_world))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
