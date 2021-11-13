mod client;
mod parser;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use client::Client;
use parser::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    done: bool,
}

#[get("/{lyric}/index.html")]
async fn index(web::Path(lyric): web::Path<String>) -> impl Responder {
    let client = Client::new();

    let body = client.get(&lyric).await;
    let ele = Parser::search_lyric(&body).await;

    HttpResponse::Ok().json(Todo {
        done: false,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
