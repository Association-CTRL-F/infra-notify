use serde::{Deserialize, Serialize};
use ureq::{Error, Response};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub embeds: Vec<Embed>,
}

impl Message {
    pub fn send(&self, url: &str) -> Result<Response, Box<Error>> {
        let res = ureq::post(url).send_json(self)?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Embed {
    color: u64,
    title: String,
    description: String,
    thumbnail: Thumbnail,
    timestamp: String,
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Field {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FieldBuilder {
    name: Option<String>,
    value: Option<String>,
    inline: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Thumbnail {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EmbedBuilder {
    color: Option<u64>,
    title: Option<String>,
    description: Option<String>,
    thumbnail: Option<Thumbnail>,
    timestamp: Option<String>,
    fields: Option<Vec<Field>>,
}

impl FieldBuilder {
    pub fn new() -> FieldBuilder {
        FieldBuilder::default()
    }

    pub fn name<T: ToString>(mut self, name: T) -> FieldBuilder {
        self.name = Some(name.to_string());
        self
    }

    pub fn value<T: ToString>(mut self, value: T) -> FieldBuilder {
        self.value = Some(value.to_string());
        self
    }

    pub fn build(self) -> Field {
        let mut result = Field::default();

        if let Some(name) = self.name {
            result.name = name;
        }
        if let Some(value) = self.value {
            result.value = value;
        }
        match self.inline {
            true => result.inline = true,
            false => result.inline = false,
        }
        result
    }
}

impl EmbedBuilder {
    pub fn new() -> EmbedBuilder {
        EmbedBuilder::default()
    }

    pub fn color(mut self, color: u64) -> EmbedBuilder {
        self.color = Some(color);
        self
    }

    pub fn title<T: ToString>(mut self, name: T) -> EmbedBuilder {
        self.title = Some(name.to_string());
        self
    }

    pub fn thumbnail<T: ToString>(mut self, url: T) -> EmbedBuilder {
        self.thumbnail = Some(Thumbnail {
            url: url.to_string(),
        });
        self
    }

    pub fn timestamp(mut self, timestamp: String) -> EmbedBuilder {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn fields(mut self, fields: Vec<Field>) -> EmbedBuilder {
        self.fields = Some(fields);
        self
    }

    pub fn build(self) -> Embed {
        let mut result = Embed::default();

        if let Some(color) = self.color {
            result.color = color;
        }

        if let Some(title) = self.title {
            result.title = title;
        }

        if let Some(description) = self.description {
            result.description = description;
        }

        if let Some(thumbnail) = self.thumbnail {
            result.thumbnail = thumbnail;
        }

        if let Some(timestamp) = self.timestamp {
            result.timestamp = timestamp;
        }

        if let Some(fields) = self.fields {
            result.fields = fields;
        }

        result
    }
}
