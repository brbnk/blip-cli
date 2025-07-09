use domain::constants;
use serde::{Deserialize, Serialize};

use crate::types::input_message::{InputMessageContent, InputMessageMetadata, MediaContent};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessage {
    #[serde(rename = "type")]
    pub message_type: String,

    #[serde(rename = "content")]
    pub content: InputMessageContent,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "metadata")]
    pub metadata: InputMessageMetadata,
}

impl InputMessage {
    pub fn new(input: &str) -> Self {
        let is_image = input.contains(constants::IMAGE_FLAG);
        let is_video = input.contains(constants::VIDEO_FLAG);
        let is_media = is_image || is_video;

        let message_type = if is_media { constants::MEDIA_TYPE } else { constants::TEXT_TYPE };
        let content = if is_media { 
            InputMessageContent::Media(MediaContent::new(is_image))
        } else {
            InputMessageContent::Text(input.to_string())
        };

        Self {
            message_type: message_type.to_string(),
            content: content,
            id: "85cca1e4-eb3e-4b77-9dfc-e408d2356e61".to_string(),
            from: constants::DEFAULT_CONTACT_IDENTITY.to_string(),
            to: constants::DEFAULT_TO.to_string(),
            metadata: InputMessageMetadata::new(),
        }
    }
}
