use fast_magic_string::error::{Error, MsErrType};

#[inline]
pub fn to_napi_error(err: Error) -> napi::Error {
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
