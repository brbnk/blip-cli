use std::{fs::{self, File}, io::{Result, Write}};
use domain::{constants, traits::file_handler::Writer};

pub struct DataFile {
  pub tenant: String,

  pub bot_id: String,

  pub file_name: String,

  pub content: String
}

impl Writer for DataFile {
    fn write(&self) -> Result<()>{
        let mut path = format!("{}/{}", constants::DATA_FOLDER, self.tenant);

        if !self.bot_id.is_empty() {
            path = format!("{}/{}", path, self.bot_id);
        }
        
        fs::create_dir_all(&path).expect("create data folder");

        let file_path = format!("{}/{}", path, self.file_name);
        let mut file = File::create(file_path)?;

        file.write_all(self.content.as_bytes())
            .expect("create file");

        Ok(())
    }
}