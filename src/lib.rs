use serde::{Deserialize, Serialize};
use ureq::{Error, Response};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    embeds: Vec<Embed>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Embed {
    color: u64,
    title: String,
    description: String,
    thumbnail: Thumbnail,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thumbnail {
    url: String,
}

impl Message {
    pub fn set_timestamp(&mut self, timestamp: &str) {
        self.embeds
            .iter_mut()
            .for_each(|embed| embed.timestamp = timestamp.to_string());
    }
}

pub const UPLOAD_SUCCESS: &str = r#"{"embeds":[{
"color":5753130,
"title":"VPS BACKUP",
"description":"La sauvegarde a bien été effectuée",
"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":""}]}"#;

pub const UPLOAD_FAILURE: &str = r#"{"embeds":[{
"color":13195050,
"title":"VPS BACKUP",
"description":"La sauvegarde a rencontré un problème",
"thumbnail":{"url":"https://ctrl-f.io/assets/img/logo.png"},
"timestamp":""}]}"#;

pub fn send(msg: &Message, url: &str) -> Result<Response, Box<Error>> {
    let res = ureq::post(url).send_json(msg)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_consts() {
        let consts: Vec<&str> = vec![UPLOAD_SUCCESS, UPLOAD_FAILURE];

        assert!(consts
            .iter()
            .all(|s| serde_json::from_str::<Message>(&s).is_ok()))
    }
}
