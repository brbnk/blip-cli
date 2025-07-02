use domain::{
    constants,
    traits::file_handler::{PathBuilder, Reader, Writer},
};
use std::{
    fs::{self, File},
    io::{Read, Result, Write},
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
    
    fn append_file_name(&self, path: &str, name: &str) -> String {
        format!("{}/{}", path, name)
    }
}

impl Writer for DataFile {
    fn write(&self) -> Result<()> {
        let path = self.build_path();

        fs::create_dir_all(&path).expect("create data folder");

        let file_path = self.append_file_name(&path, &self.file_name);
        let mut file = File::create(file_path)?;

        match &self.content {
            Some(c) => file.write_all(c.as_bytes()).expect("create file"),
            None => {},
        }

        Ok(())
    }
}

impl Reader for DataFile {
    fn read(&self) -> result::Result<String, String> {
        let path = self.append_file_name(&self.build_path(), &self.file_name);

        let mut file =
            File::open(&path).map_err(|e| format!("Erro ao abrir o arquivo {}: {}", path, e))?;

        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .map_err(|e| format!("Erro ao ler o arquivo: {}", e))?;

        Ok(contents)
    }
}
