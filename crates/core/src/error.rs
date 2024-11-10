use std::{io, string};

use fast_magic_string_sourcemap::error::SourcemapError;

#[derive(Debug)]
pub struct Error {
  pub err_type: MsErrType,
  pub err_msg: Option<String>,
}

impl Default for Error {
  fn default() -> Self {
    Self {
      err_type: MsErrType::Default,
      err_msg: None,
    }
  }
}

impl From<serde_json::Error> for Error {
  #[inline]
  fn from(_: serde_json::Error) -> Self {
    Error::new(MsErrType::JSON)
  }
}

impl From<io::Error> for Error {
  #[inline]
  fn from(_: io::Error) -> Self {
    Error::new(MsErrType::IO)
  }
}

impl From<SourcemapError> for Error {
  #[inline]
  fn from(err: SourcemapError) -> Self {
    match err {
      SourcemapError::Vlq(_) => Error::new(MsErrType::Vlq),
      SourcemapError::Io(_) => Error::new(MsErrType::IO),
      SourcemapError::FromUtf8Error(_) => Error::new(MsErrType::StringFromUTF8),
    }
  }
}

impl From<string::FromUtf8Error> for Error {
  #[inline]
  fn from(_: string::FromUtf8Error) -> Self {
    Error::new(MsErrType::StringFromUTF8)
  }
}

impl Error {
  pub fn new(err_type: MsErrType) -> Self {
    Self {
      err_type,
      err_msg: None,
    }
  }
  pub fn from_reason(err_type: MsErrType, reason: &str) -> Self {
    Self {
      err_type,
      err_msg: Some(String::from(reason)),
    }
  }
}

#[derive(Debug)]
pub enum MsErrType {
  Default,
  Deprecated,
  Range,
  Overwrite,
  SplitChunk,
  Type,
  JSON,
  IO,
  Vlq,
  StringFromUTF8,
  Slice,
}
