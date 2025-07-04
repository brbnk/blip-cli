use serde::de::DeserializeOwned;

pub fn deserialize<T>(str: &String) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let result = serde_json::from_str::<T>(str)
        .map_err(|e| format!("Erro ao fazer parse do JSON: {}", e))?;
    
    Ok(result)
}
