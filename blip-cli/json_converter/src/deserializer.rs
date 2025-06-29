use serde::de::DeserializeOwned;

pub fn deserialize<T>(flow: &String) -> Result<T, String> where T: DeserializeOwned
{
    let flow = serde_json::from_str::<T>(flow).map_err(|e| format!("Erro ao fazer parse do JSON: {}", e))?;
    Ok(flow)
}
