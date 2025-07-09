use std::io::Result;

pub trait Writer {
  fn write(&self) -> Result<()>;
}