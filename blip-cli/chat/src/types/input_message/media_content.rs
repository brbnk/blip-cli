use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaContent {
    #[serde(rename = "type")]
    pub media_type: String,

    #[serde(rename = "size")]
    pub size: u32,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "title")]
    pub title: String,
}

impl MediaContent {
    pub fn new(is_image: bool) -> Self {
        let media_type = if is_image { "image/jpeg" } else { "video/mp4" };
        let extension = if is_image { "jpeg" } else { "mp4" };

        Self {
            media_type: media_type.to_string(),
            size: 99999,
            uri: format!("https://www.media.com/media.{extension}"),
            title: "".to_string(),
        }
    }
}
