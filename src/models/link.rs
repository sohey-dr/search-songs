use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    url: String,
    title: String,
}