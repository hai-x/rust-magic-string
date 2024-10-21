use std::{io, string};

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

impl From<vlq::Error> for Error {
  #[inline]
  fn from(_: vlq::Error) -> Self {
    Error::new(MsErrType::Vlq)
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

impl From<Error> for napi::Error {
  fn from(err: Error) -> Self {
    let mut reason = String::from("<rust-magic-string> ");
    match err.err_type {
      MsErrType::Default => {
        reason.push_str("Default error (not expected to occur)");
      }
      MsErrType::Deprecated => {
        reason.push_str("Deprecated api error");
      }
      MsErrType::Range => {
        reason.push_str("Index out of range");
      }
      MsErrType::Overwrite => {
        reason.push_str("Overwrite error");
      }
      MsErrType::Type => {
        reason.push_str("TypeError");
      }
      MsErrType::SplitChunk => {
        reason.push_str("Split chunk error");
      }
      MsErrType::JSON => {
        reason.push_str("Json serialize error");
      }
      MsErrType::IO => {
        reason.push_str("IO error");
      }
      MsErrType::Vlq => {
        reason.push_str("Vlq encode error");
      }
      MsErrType::StringFromUTF8 => {
        reason.push_str("String from utf-8 error");
      }
      MsErrType::Slice => {
        reason.push_str("Slice error");
      }
    }
    reason.push_str(": ");
    reason.push_str(err.err_msg.unwrap_or_default().as_str());
    napi::Error::new(napi::Status::GenericFailure, reason)
  }
}
