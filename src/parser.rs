use scraper::{Html, Selector};
use crate::models::link::Link;

pub struct Parser {}

impl Parser {
    pub async fn search_lyric(body: &str) -> Vec<Link> {
        let document = Html::parse_document(&body);
        let selector = Selector::parse(".kCrYT > a").unwrap();

        let mut links = vec![];
        for node in document.select(&selector) {
            let href = node.value().attr("href").unwrap();

            // モジュールであるscraperの場合Googleをスクレイピングすると無駄な文字列が入るので削除
            let last_index = href.find("&sa=").unwrap();
            let url = &href[7..last_index];

            if url.contains("uta-net") || url.contains("j-lyric.net") || url.contains("utamap") {
                let link = Link {
                    url: url.to_string(),
                    title: node.text().collect::<Vec<_>>()[0].to_string(),
                };
                links.push(link);
            }
        }

        links
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_search_lyric() {
        let body = r#"
            <!DOCTYPE html>
            <html lang="ja">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <meta http-equiv="X-UA-Compatible" content="ie=edge">
                <title>UTA-NET</title>
            <head>
            <body>
                <div class="kCrYT">
                    <a href="/url?q=https://uta-net/test.com&sa=U&ved=2ahUKEwjXo5Hg_5b0AhXwrpUCHUJ8Br8QFnoECAcQAg&usg=AOvVaw2-3NlnEVmn0zcGPv-8ANKZ">test 歌詞</a>
                </div>
                <div class="kCrYT">
                    <a href="/url?q=https://utamap-net/test.com&sa=U&ved=2ahUKEwjXo5Hg_5b0AhXwrpUCHUJ8Br8QFnoECAcQAg&usg=AOvVaw2-3NlnEVmn0zcGPv-8ANKZ">testの歌詞です</a>
                </div>
            </body>
            </html>
        "#;

        let links = Parser::search_lyric(body).await;
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].url, "https://uta-net/test.com");
        assert_eq!(links[0].title, "test 歌詞");
        assert_eq!(links[1].url, "https://utamap-net/test.com");
        assert_eq!(links[1].title, "testの歌詞です");
    }
}