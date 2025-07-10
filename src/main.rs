use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

async fn halo() -> impl Responder {
    HttpResponse::Ok().body("Halo aku Waskitho dari Rust Web Service!")
}

async fn info() -> impl Responder {
    HttpResponse::Ok().body("Waskitho : Ini adalah endpoint /info")
}

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn json_response() -> impl Responder {
    web::Json(Message {
        message: "Halo dari Waskitho dan endpoint JSON".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server berjalan di http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(halo))
            .route("/info", web::get().to(info))
            .route("/json", web::get().to(json_response))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}