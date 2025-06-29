use std::{fs::File, io::Read};

pub fn read_file_json_to_string(path: String) -> Result<String, String> {
    let mut file = File::open(&path)
        .map_err(|e| format!("Erro ao abrir o arquivo {}: {}", path, e))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| format!("Erro ao ler o arquivo: {}", e))?;

    Ok(contents)
}