use std::{fs::{self, File, ReadDir}, io::{Read, Write}};

use dirs::home_dir;
use domain::constants;

pub mod types;
mod deserializer;
pub use deserializer::deserialize;

pub fn resolve_path(child_path: Option<&str>) -> String {
    let mut path = home_dir().expect("Não foi possível obter o diretório home");

    path.push(constants::ROOT_FOLDER);

    match child_path {
        Some(p) => path.push(p),
        None => {}
    }

    path.to_string_lossy().into_owned()
}

pub fn read_dir(path: &str) -> ReadDir {
    fs::read_dir(path).expect("diretório lido")
}

pub fn create_dir(path: &str) {
    fs::create_dir_all(path).expect("diretório criado")
}

pub fn create_file(path: &str, file_name: &str, file_content: &Option<String>) {
    let mut file = File::create(format!("{}/{}", path, file_name)).expect("file created");

    match &file_content {
        Some(c) => file.write_all(c.as_bytes()).expect("create file"),
        None => {}
    }
}

pub fn read_file(path: &str, file_name: &str) -> Result<String, String> {
    let file_path = format!("{}/{}", path, file_name);

    let mut file = File::open(&file_path)
        .map_err(|e| format!("Erro ao abrir o arquivo {}: {}", path, e))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| format!("Erro ao ler o arquivo: {}", e))?;

    Ok(contents)
}