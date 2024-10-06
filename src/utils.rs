use crate::error::{Error, MsErrType};
use crate::result::Result;

pub fn slice_string(s: String, start: usize, end: usize) -> String {
  s[start..end].to_owned()
}

pub fn _normalize_range(str: &str, start: i32, end: i32) -> Result<(u32, u32)> {
  let mut _start = start;
  let mut _end = end;
  let len = str.len() as i32;
  if len > 0 {
    while _start < 0 {
      _start += len;
    }
    while _end < 0 {
      _end += len;
    }
  }

  if _end > len {
    return Err(Error::from_reason(MsErrType::Range, "end is out of bounds"));
  }
  Ok((_start as u32, _end as u32))
}

#[napi(object)]
pub struct JsRegExp {
  pub global: Option<bool>,
  pub rule: String,
}

use regex::{Captures, Regex};

pub fn match_all<'a>(re: &Regex, text: &'a str, global: bool) -> (Vec<Captures<'a>>, Vec<usize>) {
  let mut matches = Vec::new();
  let mut start = 0;
  let mut offsets = Vec::new();

  while let Some(captures) = re.captures(&text[start..]) {
    offsets.push(start);
    start += captures.get(0).unwrap().end();
    matches.push(captures);
    if !global {
      return (matches, offsets);
    }
  }
  (matches, offsets)
}

pub fn get_relative_path(from: &str, to: &str) -> String {
  let from_parts: Vec<&str> = from.split(&['/', '\\'][..]).collect();
  let to_parts: Vec<&str> = to.split(&['/', '\\'][..]).collect();

  let mut from_parts_clone = from_parts.clone();
  from_parts_clone.pop();

  let mut common_length = 0;
  while common_length < from_parts_clone.len() && common_length < to_parts.len() {
    if from_parts_clone[common_length] == to_parts[common_length] {
      common_length += 1;
    } else {
      break;
    }
  }

  let mut relative_path = Vec::new();

  for _ in common_length..from_parts_clone.len() {
    relative_path.push("..");
  }

  for part in &to_parts[common_length..] {
    relative_path.push(part);
  }

  relative_path.join("/")
}
