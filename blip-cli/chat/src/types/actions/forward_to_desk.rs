use domain::chat::Executable;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardToDesk {

}

impl Executable for ForwardToDesk {
    fn execute(&self) {
        
    }
}