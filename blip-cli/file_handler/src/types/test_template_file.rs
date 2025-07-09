use std::{
    fs::{self, File},
    io::{Read, Write, Result},
};

use domain::{
    constants,
    file_handler::{PathBuilder, Writer},
};
use ui::printer;

pub struct TestTemplateFile {
    pub tenant: String,

    pub bot_id: String,

    pub content: Option<String>,
}

impl PathBuilder for TestTemplateFile {
    fn build_path(&self) -> String {
        format!(
            "./{}/{}/{}/{}",
            constants::DATA_FOLDER,
            self.tenant,
            self.bot_id,
            constants::TESTS_FOLDER
        )
    }
}

impl Writer for TestTemplateFile {
    fn write(&self) -> std::io::Result<()> {
        let path = self.build_path();

        fs::create_dir_all(&path)?;

        let count = self.count();
        let file_name = format!("test_case_{}.json", count + 1);
        let file_path = self.append_file_name(&path, &file_name);
        let mut file = File::create(&file_path)?;
        match &self.content {
            Some(c) => {
              println!();
              file.write_all(c.as_bytes())?;
              printer::print_success_message(&format!("Arquivo '{}' criado com sucesso!", file_path));
            },
            None => {}
        }

        Ok(())
    }
}

impl TestTemplateFile {
    pub fn count(&self) -> usize {
        let path = self.build_path();

        fs::read_dir(path)
            .expect("read directory")
            .filter_map(Result::ok) // ignora erros ao ler entradas
            .filter(|entry| entry.path().is_file()) // considera apenas arquivos
            .count()
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
