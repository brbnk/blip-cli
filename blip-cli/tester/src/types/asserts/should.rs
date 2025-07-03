use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Should {
  BeEqual,
  BeEmpty,
  Exist,
  BeCalled,
  NotExist,
  BeSent
}