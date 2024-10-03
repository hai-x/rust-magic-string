use std::result;

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

pub type Result<T> = result::Result<T, Error>;

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
  Range,
  Deprecated,
  Overwrite,
  Default,
}

impl From<Error> for napi::Error {
  fn from(err: Error) -> Self {
    let mut reason = String::from("<rust-magic-string> ");
    match err.err_type {
      MsErrType::Range => {
        reason.push_str("Index out of range");
      }
      MsErrType::Overwrite => {
        reason.push_str("Overwrite failed");
      }
      MsErrType::Default => {
        reason.push_str("Unexpected error occurs");
      }
      MsErrType::Deprecated => {
        reason.push_str("Deprecated api");
      }
    }
    reason.push_str(": ");
    reason.push_str(err.err_msg.unwrap_or_default().as_str());
    napi::Error::new(napi::Status::GenericFailure, reason)
  }
}
