use serde::{Deserialize, Serialize};

use crate::types::asserts::{RedirectAssert, SendMessageAssert, TrackingAssert, VariableAssert};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AssertType {
  #[serde(rename = "tracking")]
  Tracking {
    #[serde(flatten)]
    inner: TrackingAssert
  },

  #[serde(rename = "variable")]
  Variable {
    #[serde(flatten)]
    inner: VariableAssert
  },

  #[serde(rename = "redirect")]
  Redirect {
    #[serde(flatten)]
    inner: RedirectAssert
  },

  #[serde(rename = "sendMessage")]
  SendMessage {
    #[serde(flatten)]
    inner: SendMessageAssert
  },
}