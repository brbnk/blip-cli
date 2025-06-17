use std::fs::File;
use std::io::Read;

use types::flow::Flow;

pub fn parse(path: String) -> Result<Flow, String> {
    let mut file = File::open(&path)
        .map_err(|e| format!("Erro ao abrir o arquivo {}: {}", path, e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Erro ao ler o arquivo: {}", e))?;

    let flow = serde_json::from_str(&contents)
        .map_err(|e| format!("Erro ao fazer parse do JSON: {}", e))?;

    Ok(flow)
}