use napi::Error;

use crate::internal_magic_string::__internal_magic_string;

#[napi]
struct MagicString(__internal_magic_string);

#[allow(dead_code)]
#[napi]
impl MagicString {
  #[napi(constructor)]
  pub fn new(str: String) -> MagicString {
    MagicString(__internal_magic_string::new(str.as_str()))
  }

  #[napi]
  pub fn append(&mut self, input: String) -> Result<&Self, Error> {
    self.0.append(input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn append_left(&mut self, index: u32, input: String) -> Result<&Self, Error> {
    self.0.append_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn insert_left(&mut self, index: u32, input: String) -> Result<&Self, Error> {
    self.0.append_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn append_right(&mut self, index: u32, input: String) -> Result<&Self, Error> {
    self.0.append_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend(&mut self, input: String) -> Result<&Self, Error> {
    self.0.prepend(input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_left(&mut self, index: u32, input: String) -> Result<&Self, Error> {
    self.0.prepend_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_right(&mut self, index: u32, input: String) -> Result<&Self, Error> {
    self.0.prepend_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn insert_right(&mut self, index: u32, input: String) -> Result<&Self, Error> {
    self.0.prepend_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn trim(&mut self, char_type: Option<String>) -> Result<&Self, Error> {
    self.0.trim(char_type.as_deref());
    Ok(self)
  }

  #[napi]
  pub fn trim_lines(&mut self) -> Result<&Self, Error> {
    self.0.trim_lines();
    Ok(self)
  }

  #[napi]
  pub fn trim_start(&mut self, char_type: Option<String>) -> Result<&Self, Error> {
    self.0.trim_start(char_type.as_deref());
    Ok(self)
  }

  #[napi]
  pub fn trim_end(&mut self, char_type: Option<String>) -> Result<&Self, Error> {
    self.0.trim_end(char_type.as_deref());
    Ok(self)
  }

  #[napi(js_name = "move")]
  pub fn _move(&mut self, start: u32, end: u32, index: u32) -> Result<&Self, Error> {
    self.0._move(start, end, index)?;
    Ok(self)
  }

  #[napi]
  pub fn to_string(&mut self) -> String {
    self.0.to_string()
  }
}
