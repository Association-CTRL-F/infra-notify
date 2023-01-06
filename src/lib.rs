use reqwest::blocking::{Client, Response};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Message {
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embeds>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Embeds {
    pub author: Option<Author>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub color: Option<u32>,
    pub fields: Option<Vec<Fields>>,
    pub thumbnail: Option<Thumbnail>,
    pub image: Option<Image>,
    pub footer: Option<Footer>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Author {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Fields {
    pub name: Option<String>,
    pub value: Option<String>,
    pub inline: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Thumbnail {
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Image {
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Footer {
    pub text: Option<String>,
    pub icon_url: Option<String>,
}

pub fn send(msg: &Message, url: &str) -> Result<Response, Error> {
    let res = Client::new().post(url).json(msg).send()?;
    Ok(res)
}

//pub fn handle_err(res: Error) {
//    if res.is_status() {
//        match res.status() {
//            StatusCode::BAD_REQUEST => println!("")
//        }
//    }
//}
