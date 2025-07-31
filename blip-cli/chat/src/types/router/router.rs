use contexts::system;
use domain::{constants, file_handler::Reader};
use file_handler::{deserialize, types::DataFile};
use serde::{Deserialize, Serialize};

use crate::types::router::RouterChild;

#[derive(Serialize, Deserialize, Debug)]
pub struct Router {
    pub tenant: String,

    pub identifier: String
}

impl Router {
    pub fn new(tenant: &str, identifier: &str) -> Self {
        system::set_router_mode();
        system::set_router_id(&identifier);
        
        read(tenant, identifier);
        
        Self { 
            tenant: tenant.to_owned(), 
            identifier: identifier.to_owned() 
        }
    }

    pub fn deserialize(&self) -> Option<Vec<RouterChild>> {
        let serialized = system::get_router_children();

        let children: Option<Vec<RouterChild>> = match serialized {
            Some(s) => deserialize(&s).expect("deserialized children"),
            None => panic!("serialized children not found"),
        };

        children
    }

    pub fn get_default_child(&self) -> Option<RouterChild> {
        let default: Option<RouterChild> = match self.deserialize() {
            Some(rc) => rc.into_iter().find(|c| c.is_default),
            None => None,
        };

        default
    }

    pub fn get_child_by_service(&self, service: &str) -> Option<RouterChild> {
        let child: Option<RouterChild> = match self.deserialize() {
            Some(rc) => rc.into_iter().find(|c| c.service.eq(service)),
            None => None,
        };

        child
    }
}

fn read(tenant: &str, identifier: &str) {
    let file = DataFile {
        tenant: tenant.to_owned(),
        bot_id: Some(identifier.to_owned()),
        file_name: constants::ROUTER_CHILDREN_FILE_NAME.to_owned(),
        content: None,
    };

    let serialized = file.read();
    
    match serialized {
        Ok(content) => system::set_router_children(&content),
        Err(_) => panic!("it was not possible to read router file"),
    }
}