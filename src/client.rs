use actix_web::client::Client as ActixClient;

pub struct Client {
    actix_client: ActixClient,
}

impl Client {
    pub fn new() -> Self {
        Self {
            actix_client: ActixClient::default(),
        }
    }

    pub async fn get(&self, path: &str) -> String {
        // NOTE: Rustで日本語の入っているURLはアクセスできないのでpathは英語であること。あとでちゃんと調べる
        let url = format!("https://www.google.com/search?q={}+kashi", path);
        let mut response = self.actix_client.get(url).send().await.unwrap();
        let byte = response.body().await.unwrap();
        let body = byte.iter().map(|&s| s as char).collect::<String>();
        println!("{}", body);

        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn get_html() {
        let client = Client::new();
        let body = client.get("kashi").await;
        assert_eq!(body.contains("<!doctype html>"), true);
    }

    #[actix_rt::test]
    async fn use_path() {
        let client = Client::new();
        let body = client.get("we+just+gifted").await;
        assert_eq!(body.contains("we just gifted"), true);
    }
}