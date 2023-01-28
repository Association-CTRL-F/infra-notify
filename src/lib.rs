use serde::{Deserialize, Serialize};
use ureq::{Error, Response};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    embeds: Vec<Embed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    color: u64,
    title: String,
    description: String,
    thumbnail: Thumbnail,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    url: String,
}

impl Message {
    pub fn set_timestamp(&mut self, timestamp: String) {
        self.embeds[0].timestamp = timestamp;
    }
}

pub const DUMP_SUCCESS: &str = r#"{"embeds":[{"color":5753130,
"title":"VPS BACKUP",
"description":"L'exportation a bien été effectuée",
"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":""}]}"#;

pub const DUMP_FAILURE: &str = r#"{"embeds":[{"color":16711680,
"title":"VPS BACKUP",
"description":"L'exportation a rencontré un problème",
"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":""}]}"#;

pub const UPLOAD_SUCCESS: &str = r#"{"embeds":[{"color":5753130,
"title":"VPS BACKUP",
"description":"La sauvegarde a bien été effectuée",
"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":""}]}"#;

pub const UPLOAD_FAILURE: &str = r#"{"embeds":[{"color":16711680,
"title":"VPS BACKUP",
"description":"La sauvegarde a rencontré un problème",
"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":""}]}"#;

pub fn send(msg: &Message, url: &str) -> Result<Response, Error> {
    let res = ureq::post(url).send_json(msg)?;
    Ok(res)
}
