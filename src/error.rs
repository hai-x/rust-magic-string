#[derive(Debug)]
pub enum MagicStringError {
  SplitPointIndexError,
  MoveSelectionError,
  MoveStartLargerError,
}

impl From<MagicStringError> for napi::Error {
  fn from(reason: MagicStringError) -> Self {
    match reason {
      MagicStringError::SplitPointIndexError => {
        return napi::Error::new(
          napi::Status::GenericFailure,
          "Failed to split chunk exceed index",
        );
      }
      MagicStringError::MoveSelectionError => {
        return napi::Error::new(
          napi::Status::GenericFailure,
          "Cannot move a selection inside itself",
        );
      }
      MagicStringError::MoveStartLargerError => {
        return napi::Error::new(
          napi::Status::GenericFailure,
          "Start must smaller than End when moving",
        );
      }
    }
  }
}
