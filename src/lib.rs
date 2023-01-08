use reqwest::blocking::{Client, Response};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub embeds: Vec<Embed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    color: u64,
    title: String,
    description: String,
    thumbnail: Thumbnail,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    url: String,
}

impl Message {
    pub fn set_timestamp(&mut self, timestamp: String) -> &Message {
        self.embeds[0].timestamp = timestamp;
        self
    }
}

pub const DUMP_SUCCESS: &str = r#"{"embeds":[{"color":5753130,
"title":"VPS BACKUP",
"description":"Le dump a bien été effectuée",
"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":"2023-01-07T22:11:25.693Z"}]}"#;

pub const DUMP_FAILURE: &str = r#"{"embeds":[{"color":16711680,
"title":"VPS BACKUP",
"description":"Le dump de la database à rencontré un problème",
"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":"2023-01-07T22:11:25.693Z"}]}"#;

pub const UPLOAD_SUCCESS: &str = r#"{"embeds":[{"color":5753130,
"title":"VPS BACKUP",
"description":"La sauvegarde vers Gdrive a bien été effectuée"
,"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":"2023-01-07T22:11:25.693Z"}]}"#;

pub const UPLOAD_FAILURE: &str = r#"{"embeds":[{"color":16711680,
"title":"VPS BACKUP",
"description":"La sauvegarde vers Gdrive a rencontré un problème"
,"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":"2023-01-07T22:11:25.693Z"}]}"#;

pub fn send(msg: &Message, url: &str) -> Result<Response, Error> {
    let res = Client::new().post(url).json(msg).send()?;
    Ok(res)
}
