#[derive(Debug)]
pub struct Match {
  pub(crate) matched: String,
  pub(crate) start: usize,
  pub(crate) end: usize,
  pub(crate) len: usize,
}

use regex::Regex;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
  static ref REGEX_CACHE: Mutex<HashMap<String, Regex>> = Mutex::new(HashMap::new());
}

pub fn rx_new(pattern_str: String) -> Regex {
  let pattern = format!(r"{}", pattern_str);
  let mut cache = REGEX_CACHE.lock().unwrap();
  if let Some(regex) = cache.get(&pattern) {
    return regex.clone();
  }
  let regex = Regex::new(&pattern).unwrap();
  cache.insert(pattern.clone(), regex.clone());
  regex
}

pub fn match_fn(re: &Regex, text: &str, global: bool) -> Vec<Match> {
  let mut matches = Vec::new();
  let mut start = 0;
  while let Some(captures) = re.captures(&text[start..]) {
    let matched_str = captures.get(0).unwrap().as_str().to_string();
    let start_pos = start + captures.get(0).unwrap().start();
    let end_pos = start + captures.get(0).unwrap().end();

    matches.push(Match {
      matched: matched_str,
      start: start_pos,
      end: end_pos,
      len: captures.get(0).unwrap().len(),
    });

    start += captures.get(0).unwrap().end(); // 更新起始位置

    if !global {
      return matches;
    }
  }

  matches
}
