use std::{collections::HashMap, sync::Mutex};

use regex::Regex;

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
