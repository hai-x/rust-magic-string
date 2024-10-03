use crate::{
  error::{Error, MsErrType},
  internal_magic_string::{OverwriteOptions, __internal_magic_string},
};

use napi::Result;

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
  pub fn insert(&mut self) -> Result<()> {
    Err(
      Error::from_reason(
        MsErrType::Deprecated,
        "magicString.insert(...) is deprecated. Use prependRight(...) or appendLeft(...)",
      )
      .into(),
    )
  }

  #[napi]
  pub fn append(&mut self, input: String) -> Result<&Self> {
    self.0.append(input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn append_left(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.append_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn insert_left(&mut self, index: u32, input: String) -> Result<&Self> {
    println!("magicString.insertLeft(...) is deprecated. Use magicString.appendLeft(...) instead");
    self.0.append_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn append_right(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.append_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend(&mut self, input: String) -> Result<&Self> {
    self.0.prepend(input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_left(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.prepend_left(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn prepend_right(&mut self, index: u32, input: String) -> Result<&Self> {
    self.0.prepend_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn insert_right(&mut self, index: u32, input: String) -> Result<&Self> {
    println!(
      "magicString.insertRight(...) is deprecated. Use magicString.prependRight(...) instead"
    );
    self.0.prepend_right(index, input.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn trim(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.0.trim(char_type.as_deref());
    Ok(self)
  }

  #[napi]
  pub fn trim_lines(&mut self) -> Result<&Self> {
    self.0.trim_lines();
    Ok(self)
  }

  #[napi]
  pub fn trim_start(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.0.trim_start(char_type.as_deref());
    Ok(self)
  }

  #[napi]
  pub fn trim_end(&mut self, char_type: Option<String>) -> Result<&Self> {
    self.0.trim_end(char_type.as_deref());
    Ok(self)
  }

  #[napi(js_name = "move")]
  pub fn _move(&mut self, start: i32, end: i32, index: u32) -> Result<&Self> {
    self.0._move(start, end, index)?;
    Ok(self)
  }

  #[napi]
  pub fn remove(&mut self, start: i32, end: i32) -> Result<&Self> {
    self.0.remove(start, end)?;
    Ok(self)
  }

  #[napi(ts_args_type = r"
    start: number,
    end: number,
    content: string,
    options?: OverwriteOptions
  ")]
  pub fn overwrite(
    &mut self,
    start: i32,
    end: i32,
    content: String,
    options: Option<OverwriteOptions>,
  ) -> Result<&Self> {
    self.0.overwrite(start, end, content.as_str(), options)?;
    Ok(self)
  }

  #[napi(ts_args_type = r"
    start: number,
    end: number,
    content: string,
    options?: OverwriteOptions
  ")]
  pub fn update(
    &mut self,
    start: i32,
    end: i32,
    content: String,
    options: Option<OverwriteOptions>,
  ) -> Result<&Self> {
    self.0.update(start, end, content.as_str(), options)?;
    Ok(self)
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  #[napi]
  pub fn to_string(&mut self) -> String {
    self.0.to_string()
  }

  #[napi]
  pub fn has_changed(&self) -> bool {
    self.0.has_changed()
  }

  #[napi]
  pub fn clone(&self) -> MagicString {
    let ms = self.0.clone();
    MagicString(ms)
  }

  #[napi]
  pub fn snip(&mut self, start: i32, end: i32) -> Result<MagicString> {
    let ms = self.0.snip(start, end)?;
    Ok(MagicString(ms))
  }

  #[napi]
  pub fn reset(&mut self, start: i32, end: i32) -> Result<&Self> {
    self.0.reset(start, end)?;
    Ok(self)
  }

  #[napi]
  pub fn replace(&mut self, search_value: String, replacement: String) -> Result<&Self> {
    self
      .0
      .replace(search_value.as_str(), replacement.as_str())?;
    Ok(self)
  }

  #[napi]
  pub fn replace_all(&mut self, search_value: String, replacement: String) -> Result<&Self> {
    self
      .0
      .replace_all(search_value.as_str(), replacement.as_str())?;
    Ok(self)
  }
}
