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

    pub async fn get(&self) -> String {
        let url = "https://www.google.com/search?q=%E5%90%9B%E3%81%8C%E5%A5%BD%E3%81%8D+%E6%AD%8C%E8%A9%9E";
        let mut response = self.actix_client.get(url).send().await.unwrap();
        let byte = response.body().await.unwrap();
        let body = byte.iter().map(|&s| s as char).collect::<String>();

        body
    }
}
