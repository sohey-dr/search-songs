use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{lyric}/index.html")]
async fn index(web::Path((lyric)): web::Path<(String)>) -> impl Responder {
    format!("この {} を調べます！", lyric)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
