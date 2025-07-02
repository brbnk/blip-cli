use std::{fs::{self, File}, io::Write};

use domain::{constants, traits::file_handler::Writer};

pub struct TestTemplateFile {
  pub tenant: String,

  pub bot_id: String,

  pub content: String
}

impl Writer for TestTemplateFile {
    fn write(&self) -> std::io::Result<()> {
        let path = format!(
          "./{}/{}/{}/{}",
          constants::DATA_FOLDER,
          self.tenant,
          self.bot_id,
          constants::TESTS_FOLDER
        );

        fs::create_dir_all(&path)?;
        
        let file_path = format!("{}/{}", path, "test_case.json");
        let mut file = File::create(file_path)?;

        file.write_all(self.content.as_bytes())?;

        Ok(())
    }
}