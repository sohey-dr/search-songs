
mod client;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use client::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
    done: bool,
}

#[get("/{lyric}/index.html")]
async fn index(web::Path(lyric): web::Path<String>) -> impl Responder {
    let client = Client::new();

    let bytes = client.get().await;
    let body = &bytes.iter().map(|&s| s as char).collect::<String>();
    let ele = search_lyric(body).await;

    HttpResponse::Ok().json(Todo {
        content: ele,
        done: false,
    })
}

async fn search_lyric(body: &str) -> String {
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".kCrYT > a").unwrap();

    for node in document.select(&selector) {
        let href = node.value().attr("href").unwrap();

        // モジュールであるscraperの場合Googleをスクレイピングすると無駄な文字列が入るので削除
        let last_index = href.find("&sa=").unwrap();
        let url = &href[7..last_index];

        if url.contains("uta-net") || url.contains("j-lyric.net") || url.contains("utamap") {
            println!("{:?}", node.text().collect::<Vec<_>>()[0]);
            println!("{:?}", url);
        }
    }

    return String::from("Hello, world!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
