pub trait Manager: Send + Sync {
  fn get(&self, key: &str) -> Option<String>;
  fn set(&self, key: &str, value: &str);
  fn reset(&self);
}