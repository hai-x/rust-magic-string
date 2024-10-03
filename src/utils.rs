use crate::error::{Error, MsErrType, Result};

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
