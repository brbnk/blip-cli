use std::{
    fs::{self, read_dir, File},
    io::{Read, Result},
};

use domain::{
    constants,
    file_handler::{PathBuilder, Writer},
};
use ui::printer;

use crate::{create_dir, create_file, resolve_path};

pub struct TestTemplateFile {
    pub tenant: String,

    pub bot_id: String,

    pub content: Option<String>,
}

impl PathBuilder for TestTemplateFile {
    fn build_path(&self) -> String {
        resolve_path(Some(&format!(
            "{}/{}/{}/{}",
            constants::DATA_FOLDER,
            self.tenant,
            self.bot_id,
            constants::TESTS_FOLDER
        )))
    }
}

impl Writer for TestTemplateFile {
    fn write(&self) -> std::io::Result<()> {
        let path = self.build_path();
        let file_name = format!("test_case_{}.json", self.count() + 1);
        
        create_dir(&path);
        create_file(&path, &file_name, &self.content);

        printer::print_success_message(&format!("Arquivo '{}' criado com sucesso!", file_name));

        Ok(())
    }
}

impl TestTemplateFile {
    pub fn count(&self) -> usize {
        let path = self.build_path();

        let dir = read_dir(path);

        match dir {
            Ok(d) => d
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .count(),
            Err(_) => 0,
        }
    }

    pub fn read_files(&self) -> Result<Vec<String>> {
        let path = self.build_path();

         fs::read_dir(path)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| {
                let mut contents = String::new();
                File::open(entry.path())?.read_to_string(&mut contents)?;
                Ok(contents)
            })
            .collect()
    }
}
