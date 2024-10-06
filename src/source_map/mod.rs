/**
 * Most of the implementations refer to
 * https://github.com/h-a-n-a/magic-string-rs/blob/main/core/src/source_map.rs
 */

/**
 * SourceMap Spec
 * https://docs.google.com/document/d/1U1RGAehQwRypUTovF1KRlpiOFze0b-_2gc6fAH0KY0k/edit?pli=1#heading=h.mofvlxcwqzej
 */
pub mod mappings;
use base64::{engine::general_purpose, Engine};
use mappings::{serialize_mappings, Mappings};

use napi::Result;

use crate::error::Error;
use crate::result::Result as InnerResult;

pub static SOURCEMAP_VERSION: u8 = 3;

#[napi]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct DecodedMap {
  pub version: u8,
  pub file: Option<String>,
  pub source_root: Option<String>,
  pub sources: Vec<String>,
  pub sources_content: Option<Vec<String>>,
  pub names: Vec<String>,
  #[napi(ts_type = "Array<number>")]
  pub mappings: Mappings,
  pub x_google_ignoreList: Option<Vec<u8>>,
}

#[napi]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct SourceMap {
  pub version: u8,
  pub file: Option<String>,
  pub source_root: Option<String>,
  pub sources: Vec<String>,
  pub sources_content: Option<Vec<String>>,
  pub names: Vec<String>,
  pub mappings: String,
  pub x_google_ignoreList: Option<Vec<u8>>,
}

#[napi]
impl SourceMap {
  pub fn new(
    mappings: &str,
    file: Option<&str>,
    names: Vec<&str>,
    sources_content: Vec<Option<&str>>,
    source_root: Option<&str>,
    sources: Vec<Option<&str>>,
    #[allow(non_snake_case)] x_google_ignoreList: Option<Vec<u8>>,
  ) -> Self {
    Self {
      version: SOURCEMAP_VERSION,
      mappings: String::from(mappings),
      file: file.map(|f| f.to_owned()),
      names: names.iter().map(|&n| n.to_owned()).collect::<Vec<String>>(),
      sources_content: sources_content
        .iter()
        .map(|s| s.map(|s| s.to_owned()))
        .collect(),
      source_root: source_root.map(|s| s.to_owned()),
      sources: sources
        .iter()
        .map(|s| s.map(|s| s.to_owned()))
        .flatten()
        .collect(),
      x_google_ignoreList,
    }
  }

  // private fn
  pub(crate) fn from_decoded_map(
    DecodedMap {
      version,
      file,
      mappings,
      names,
      sources_content,
      sources,
      source_root,
      x_google_ignoreList,
    }: DecodedMap,
  ) -> InnerResult<Self> {
    Ok(Self {
      version,
      file,
      mappings: serialize_mappings(&mappings)?,
      names,
      sources_content,
      sources,
      source_root,
      x_google_ignoreList,
    })
  }

  #[napi]
  pub fn to_string(&self) -> Result<String> {
    let s = serde_json::to_string(self);
    match s {
      Ok(s) => Ok(s),
      Err(e) => Err(Error::into(e.into())),
    }
  }

  #[napi]
  pub fn to_url(&self) -> Result<String> {
    let str = self.to_string()?;
    Ok(format!(
      "data:application/json;charset=utf-8;base64,{}",
      general_purpose::STANDARD.encode(str),
    ))
  }
}
