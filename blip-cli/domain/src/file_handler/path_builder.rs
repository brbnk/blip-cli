pub trait PathBuilder {
  fn build_path(&self) -> String;
  
  fn append_file_name(&self, path:&str, name: &str) -> String {
    format!("{}/{}", path, name)
  }
}