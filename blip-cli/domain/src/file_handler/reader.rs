pub trait Reader {
  fn read(&self) -> Result<String, String>;
}