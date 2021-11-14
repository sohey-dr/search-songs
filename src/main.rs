mod client;
mod parser;
mod models;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use client::Client;
use parser::Parser;
use models::link::Links;

#[get("search/{lyric}")]
async fn index(web::Path(lyric): web::Path<String>) -> impl Responder {
    let client = Client::new();

    let body = client.get(&lyric).await;
    let links = Parser::search_lyric(&body).await;

    HttpResponse::Ok().json(
        Links {
            links: links,
        },
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
