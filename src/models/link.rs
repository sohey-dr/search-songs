use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
    pub title: String,
}