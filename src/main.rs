use actix_web::{get, web, client, App, HttpServer, Responder, HttpResponse};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
    done: bool,
}

#[get("/{lyric}/index.html")]
async fn index(web::Path(lyric): web::Path<String>) -> impl Responder {
    let body = client::get("https://www.google.com/search?q=%E5%90%9B%E3%81%8C%E5%A5%BD%E3%81%8D+%E6%AD%8C%E8%A9%9E")
            .finish().unwrap()
            .send().await.unwrap();

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
    let ele = String::from("Hello, world!");

    HttpResponse::Ok().json(Todo {
        content: ele,
        done: false,
    })
}

async fn search_lyric() -> String {
    let body = reqwest::blocking::get("https://www.google.com/search?q=%E5%90%9B%E3%81%8C%E5%A5%BD%E3%81%8D+%E6%AD%8C%E8%A9%9E").unwrap().text().unwrap();
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
