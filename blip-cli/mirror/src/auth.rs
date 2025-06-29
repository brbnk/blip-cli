use std::env;
static TOKEN: &str = "BLIP_PORTAL_TOKEN";

pub fn get_token() -> Option<String> {
  if let Ok(token) = env::var(TOKEN) {
    Some(token)
  }
  else {
      println!(
        "Configure a variável de ambiente ${} com o token obtido no Portal.", 
        TOKEN);
      None
  }
}