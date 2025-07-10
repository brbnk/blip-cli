use crate::{create_dir, create_file, read_file};
use domain::{
    constants,
    file_handler::{PathBuilder, Reader, Writer},
};
use std::{
    io::{Result},
    result,
};

pub struct DataFile {
    pub tenant: String,
    pub bot_id: Option<String>,
    pub file_name: String,
    pub content: Option<String>,
}

impl PathBuilder for DataFile {
    fn build_path(&self) -> String {
        let path = format!("{}/{}", constants::DATA_FOLDER, self.tenant);

        match &self.bot_id {
            Some(id) => format!("{}/{}", path, id),
            None => path,
        }
    }
}

impl Writer for DataFile {
    fn write(&self) -> Result<()> {
        let path = self.build_path();
        
        create_dir(&path);
        create_file(&path, &self.file_name, &self.content);

        Ok(())
    }
}

impl Reader for DataFile {
    fn read(&self) -> result::Result<String, String> {
        Ok(read_file(&self.build_path(), &self.file_name))
    }
}
