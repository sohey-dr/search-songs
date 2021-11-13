use scraper::{Html, Selector};

pub struct Parser {}

impl Parser {
    pub async fn search_lyric(body: &str) -> String {
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
}