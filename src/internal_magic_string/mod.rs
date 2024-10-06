use std::{cell::RefCell, collections::HashMap, rc::Rc, vec};

use crate::{
  error::{Error, MsErrType},
  locator::Locator,
  result::Result,
  source_map::{
    mappings::{MappingsFacade, SOURCE_INDEX},
    DecodedMap, SourceMap, SOURCEMAP_VERSION,
  },
  utils::{_normalize_range, get_relative_path, match_all, slice_string},
};

pub mod chunk;
use chunk::Chunk;
use regex::{Captures, Regex};

#[derive(Clone)]
#[napi(object)]
pub struct GenerateMapOptions {
  pub file: Option<String>,
  pub source: Option<String>,
  pub source_root: Option<String>,
  pub include_content: Option<bool>,
  pub hires: Option<bool>,
}

impl Default for GenerateMapOptions {
  fn default() -> Self {
    Self {
      file: Some(String::default()),
      source_root: Some(String::default()),
      source: Some(String::default()),
      include_content: Some(false),
      hires: Some(false),
    }
  }
}

#[derive(Clone)]
#[napi(object)]
pub struct OverwriteOptions {
  pub content_only: Option<bool>,
  pub store_name: Option<bool>,
  pub overwrite: Option<bool>,
}

impl Default for OverwriteOptions {
  fn default() -> Self {
    Self {
      content_only: Some(false),
      store_name: Some(false),
      overwrite: Some(false),
    }
  }
}

#[napi(object)]
#[derive(Clone)]
pub struct MagicStringOptions {
  pub filename: Option<String>,
  pub indent_exclusion_ranges: Option<Vec<u32>>,
  pub ignore_list: Option<bool>,
}

impl Default for MagicStringOptions {
  fn default() -> Self {
    Self {
      filename: Some(String::default()),
      indent_exclusion_ranges: Some(vec![]),
      ignore_list: Some(false),
    }
  }
}

#[allow(non_camel_case_types)]
pub struct __internal_magic_string {
  pub original: String,
  pub intro: String,
  pub outro: String,

  pub start_index_chunk_map: HashMap<u32, Rc<RefCell<Chunk>>>,
  pub end_index_chunk_map: HashMap<u32, Rc<RefCell<Chunk>>>,

  last_searched_chunk: Rc<RefCell<Chunk>>,
  first_chunk: Rc<RefCell<Chunk>>,
  last_chunk: Rc<RefCell<Chunk>>,
  stored_names: Vec<String>,
  ignore_list: bool,

  _locator: Locator,
  _raw_options: MagicStringOptions,
}

impl __internal_magic_string {
  pub fn new(str: &str, options: Option<MagicStringOptions>) -> Self {
    let options = options.unwrap_or_default();
    let chunk = Rc::new(RefCell::new(Chunk::new(0, str.len() as u32, str)));
    let init_start_index_chunk_map: Vec<(u32, Rc<RefCell<Chunk>>)> = vec![(0, chunk.clone())];
    let init_end_index_chunk_map: Vec<(u32, Rc<RefCell<Chunk>>)> =
      vec![(str.len() as u32, chunk.clone())];
    Self {
      original: str.to_string(),
      intro: String::default(),
      outro: String::default(),
      start_index_chunk_map: init_start_index_chunk_map.into_iter().collect(),
      end_index_chunk_map: init_end_index_chunk_map.into_iter().collect(),

      last_searched_chunk: Rc::clone(&chunk),
      first_chunk: Rc::clone(&chunk),
      last_chunk: Rc::clone(&chunk),

      stored_names: vec![],

      ignore_list: options.ignore_list.unwrap_or_default(),
      _locator: Locator::new(str),
      _raw_options: options,
    }
  }

  pub fn append(&mut self, str: &str) -> Result<&mut Self> {
    self.outro = format!("{}{}", self.outro, str);
    Ok(self)
  }

  pub fn prepend(&mut self, str: &str) -> Result<&mut Self> {
    self.intro = format!("{}{}", str, self.intro);
    Ok(self)
  }

  pub fn append_left(&mut self, index: u32, content: &str) -> Result<&mut Self> {
    self._split(index)?;
    if let Some(chunk) = self.end_index_chunk_map.get(&index) {
      let mut chunk = chunk.borrow_mut();
      chunk.append_left(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn append_right(&mut self, index: u32, content: &str) -> Result<&mut Self> {
    self._split(index)?;
    if let Some(chunk) = self.start_index_chunk_map.get(&index) {
      let mut chunk: std::cell::RefMut<'_, Chunk> = chunk.borrow_mut();
      chunk.append_right(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn prepend_left(&mut self, index: u32, content: &str) -> Result<&mut Self> {
    self._split(index)?;
    if let Some(chunk) = self.end_index_chunk_map.get(&index) {
      let mut chunk: std::cell::RefMut<'_, Chunk> = chunk.borrow_mut();
      chunk.prepend_left(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn prepend_right(&mut self, index: u32, content: &str) -> Result<&mut Self> {
    self._split(index)?;
    if let Some(chunk) = self.start_index_chunk_map.get(&index) {
      let mut chunk: std::cell::RefMut<'_, Chunk> = chunk.borrow_mut();
      chunk.prepend_right(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn is_empty(&self) -> bool {
    self.to_string().trim().is_empty()
  }

  pub fn trim(&mut self, char_type: Option<&str>) {
    self.trim_start(char_type).trim_end(char_type);
  }

  pub fn trim_start_aborted(&mut self, char_type: Option<&str>) -> bool {
    let pat = "^".to_owned() + char_type.unwrap_or("\\s") + "+";
    let reg = Regex::new(pat.as_str()).unwrap();
    self.intro = reg.replace(&self.intro, "").to_string();
    if !self.intro.is_empty() {
      return true;
    }
    let mut cur = Some(Rc::clone(&self.first_chunk));

    while let Some(c) = cur {
      let mut _cur = c.borrow_mut();
      // let end = _cur.end;
      let aborted = _cur.trim_start(&reg);
      if aborted {
        return true;
      }
      cur = _cur.next.clone();
    }

    false
  }

  pub fn trim_start(&mut self, char_type: Option<&str>) -> &mut Self {
    self.trim_start_aborted(char_type);
    self
  }

  pub fn trim_end_aborted(&mut self, char_type: Option<&str>) -> bool {
    let pat = char_type.unwrap_or("\\s").to_owned() + "+$";
    let reg = Regex::new(pat.as_str()).unwrap();
    self.outro = reg.replace(&self.outro, "").to_string();
    if !self.outro.is_empty() {
      return true;
    }
    let mut cur = Some(Rc::clone(&self.last_chunk));

    while let Some(c) = cur {
      let mut _cur = c.borrow_mut();
      // let end = _cur.end;
      let aborted = _cur.trim_end(&reg);
      if aborted {
        return true;
      }
      cur = _cur.previous.as_ref().map(Rc::clone);
    }

    false
  }

  pub fn trim_end(&mut self, char_type: Option<&str>) -> &mut Self {
    self.trim_end_aborted(char_type);
    self
  }

  pub fn trim_lines(&mut self) -> &mut Self {
    self.trim(Some("[\\r\\n]"));
    self
  }

  pub fn _move(&mut self, start: i32, end: i32, index: u32) -> Result<&mut Self> {
    let (_start, _end) = _normalize_range(self.original.as_str(), start, end)?;

    if index >= _start && index <= _end {
      return Err(Error::from_reason(
        MsErrType::Range,
        "Index must be in the range of Start and End",
      ));
    }

    self._split(_start)?;
    self._split(_end)?;
    self._split(index)?;

    let first = self
      .start_index_chunk_map
      .get(&_start)
      .map(Rc::clone)
      .unwrap();
    let last = self.end_index_chunk_map.get(&_end).map(Rc::clone).unwrap();

    let old_left = first.borrow().clone().previous;
    let old_right = last.borrow().clone().next;

    let new_right = self.start_index_chunk_map.get(&index).map(Rc::clone);
    let new_left = match new_right.clone() {
      Some(l) => Rc::clone(&l).borrow().clone().previous,
      None => Some(Rc::clone(&self.last_chunk)),
    };

    let _old_left = old_left.clone();

    match old_left {
      Some(old_left) => {
        old_left.borrow_mut().next = old_right.clone();
      }
      None => self.first_chunk = old_right.clone().unwrap(),
    }

    match old_right {
      Some(old_right) => old_right.borrow_mut().previous = _old_left,
      None => self.last_chunk = _old_left.unwrap(),
    }

    match new_left {
      Some(new_left) => {
        new_left.borrow_mut().next = Some(Rc::clone(&first));
        first.borrow_mut().previous = Some(new_left);
      }
      None => {
        let first = Rc::clone(&first);
        self.first_chunk.borrow_mut().previous = Some(Rc::clone(&first));
        last.borrow_mut().next = Some(Rc::clone(&self.first_chunk));
        self.first_chunk = first;
      }
    }

    match new_right {
      Some(new_right) => {
        new_right.borrow_mut().previous = Some(Rc::clone(&last));
        last.borrow_mut().next = Some(new_right);
      }
      None => {
        let last = Rc::clone(&last);
        self.last_chunk.borrow_mut().next = Some(Rc::clone(&last));
        first.borrow_mut().previous = Some(Rc::clone(&self.last_chunk));
        last.borrow_mut().next = None;
        self.last_chunk = last;
      }
    }

    Ok(self)
  }

  pub fn overwrite(
    &mut self,
    start: i32,
    end: i32,
    content: &str,
    options: Option<OverwriteOptions>,
  ) -> Result<&mut Self> {
    let mut option = options.unwrap_or_default();
    option.overwrite = Some(!option.content_only.unwrap_or_default());
    self.update(start, end, content, Some(option))
  }

  pub fn update(
    &mut self,
    start: i32,
    end: i32,
    content: &str,
    options: Option<OverwriteOptions>,
  ) -> Result<&mut Self> {
    let option = options.unwrap_or_default();
    let store_name = option.store_name.unwrap_or_default();
    let content_only = option.content_only.unwrap_or_default();

    let (_start, _end) = _normalize_range(self.original.as_str(), start, end)?;

    if _start == _end {
      return Err(Error::from_reason(
        MsErrType::Range,
        "Cannot overwrite a zero-length range – use appendLeft or prependRight instead",
      ));
    }

    self._split(_start)?;
    self._split(_end)?;

    if store_name {
      let original = self.original.clone();
      self
        .stored_names
        .push(slice_string(original, _start as usize, _end as usize));
    }

    let first = self.start_index_chunk_map.get(&_start);
    let last = self.end_index_chunk_map.get(&_end);

    if first.is_some() && last.is_some() {
      let first = Rc::clone(first.unwrap());
      first.borrow_mut().edit(
        content,
        store_name,
        !option.overwrite.unwrap_or_else(|| false),
      );
      let last = Rc::clone(last.unwrap());
      let mut cur = Some(first);
      while cur.is_some() && cur.clone().unwrap() != last {
        let c = cur.as_ref().unwrap();
        if c.borrow().next.as_ref() != self.start_index_chunk_map.get(&c.borrow().end) {
          return Err(Error::from_reason(
            MsErrType::Overwrite,
            "Cannot overwrite across a split point",
          ));
        }
        let next = c.borrow().next.clone();
        next.clone().unwrap().borrow_mut().edit("", false, false);
        cur = next;
      }
    } else {
      let mut new_chunk = Chunk::new(_start, _end, "");
      new_chunk.edit(content, store_name, content_only);

      if let Some(_last) = last {
        new_chunk.previous = Some(Rc::clone(_last));
        _last.borrow_mut().next = Some(Rc::new(RefCell::new(new_chunk)))
      }
    }

    Ok(self)
  }

  pub fn remove(&mut self, start: i32, end: i32) -> Result<&Self> {
    let (_start, _end) = _normalize_range(self.original.as_str(), start, end)?;

    if _start == _end {
      return Ok(self);
    }

    self._split(_start)?;
    self._split(_end)?;

    let first = self.start_index_chunk_map.get(&_start);
    let last = self.end_index_chunk_map.get(&_end);
    let mut cur = first.and_then(|r| Some(r.to_owned()));

    if first.is_some() && last.is_some() {
      while cur.is_some() && cur.clone().unwrap().borrow().end <= _end {
        let c = Rc::clone(&cur.unwrap());
        c.borrow_mut().edit("", false, false);
        cur = c.borrow().next.clone();
      }
    }
    Ok(self)
  }

  pub fn has_changed(&self) -> bool {
    self.original != self.to_string()
  }

  pub fn clone(&self) -> __internal_magic_string {
    let mut cloned =
      __internal_magic_string::new(self.original.as_str(), Some(self._raw_options.clone()));
    cloned.first_chunk = Rc::new(RefCell::new(self.first_chunk.borrow().self_clone()));
    cloned.last_chunk = Rc::clone(&cloned.first_chunk);
    cloned.last_searched_chunk = Rc::clone(&cloned.first_chunk);

    let mut original_chunk = Some(Rc::clone(&self.first_chunk));
    let mut cloned_chunk = Some(Rc::clone(&cloned.first_chunk));

    while let Some(o) = original_chunk {
      if let Some(c) = cloned_chunk {
        // update `cloned chunk`
        cloned
          .start_index_chunk_map
          .insert(c.borrow().start, Rc::clone(&c));
        cloned
          .end_index_chunk_map
          .insert(c.borrow().end, Rc::clone(&c));

        // update `next cloned chunk`
        let original_next = o.borrow().next.clone();
        let cloned_next = if original_next.is_some() {
          let mut cloned_next = original_next.unwrap().borrow().self_clone();
          cloned_next.previous = Some(c.clone());
          Some(Rc::new(RefCell::new(cloned_next)))
        } else {
          None
        };

        // connect `cloned chunk` and `next cloned chunk`
        cloned_chunk = if cloned_next.is_some() {
          c.borrow_mut().next = cloned_next.clone();
          cloned.last_chunk = Rc::clone(&cloned_next.clone().unwrap());
          cloned_next.clone()
        } else {
          None
        }
      }
      original_chunk = o.borrow().next.clone();
    }

    cloned.intro = self.intro.clone();
    cloned.outro = self.outro.clone();

    cloned
  }

  pub fn snip(&mut self, start: i32, end: i32) -> Result<__internal_magic_string> {
    let mut cloned = self.clone();
    cloned.remove(0, start)?;
    cloned.remove(end, cloned.original.len() as i32)?;
    Ok(cloned)
  }

  pub fn reset(&mut self, start: i32, end: i32) -> Result<&Self> {
    let (_start, _end) = _normalize_range(self.original.as_str(), start, end)?;
    if _start == _end {
      return Ok(self);
    }
    self._split(_start)?;
    self._split(_end)?;
    let mut first = self.start_index_chunk_map.get(&_start).map(Rc::clone);
    while let Some(c) = first {
      c.borrow_mut().reset();
      first = if _end > c.borrow().end {
        self
          .start_index_chunk_map
          .get(&c.borrow().end)
          .map(Rc::clone)
      } else {
        None
      }
    }
    Ok(self)
  }

  pub fn _replace_regexp(
    &mut self,
    search_value: &str,
    replacement: &str,
    global: bool,
  ) -> Result<&Self> {
    let this = self as *mut Self;

    let reg = Regex::new(search_value).unwrap();
    let str = self.original.as_str();
    let matches = match_all(&reg, str, global);

    let get_replacement = |match_item: &Captures| {
      let reg = Regex::new(r"\$(\$|&|(\d+))").unwrap();
      return reg.replace(replacement, |caps: &Captures| {
        let matched = &caps[0];
        let i = &caps[1];
        match i {
          "$" => "$".to_string(),
          "&" => matched.to_string(),
          num_str => {
            if let Ok(num) = num_str.parse::<usize>() {
              if num < match_item.len() {
                return match_item.get(num).unwrap().as_str().to_string();
              } else {
                format!("${}", i)
              }
            } else {
              format!("${}", i)
            }
          }
        }
      });
    };

    for (idx, caps) in matches.0.iter().enumerate() {
      let _replacement = get_replacement(caps);
      let offset = matches.1.get(idx).unwrap_or_else(|| &0);
      let start = (caps.get(0).unwrap().start() + *offset) as i32;
      let end = (caps.get(0).unwrap().end() + *offset) as i32;
      unsafe {
        (*this).overwrite(start, end, _replacement.into_owned().as_str(), None)?;
      }
    }
    Ok(self)
  }

  pub fn _replace_string(&mut self, search_value: &str, replacement: &str) -> Result<&Self> {
    let start = self.original.find(search_value);

    if let Some(start) = start {
      self.overwrite(
        start as i32,
        (start + search_value.len()) as i32,
        replacement,
        None,
      )?;
    }

    Ok(self)
  }

  pub fn _replace_all_string(&mut self, search_value: &str, replacement: &str) -> Result<&Self> {
    let mut start = self.original.find(search_value);
    let mut offset: usize = 0;
    while let Some(_start) = start {
      let _start = _start + offset;
      offset = _start + search_value.len();
      self.overwrite(_start as i32, offset as i32, replacement, None)?;
      start = if offset <= self.original.len() {
        self.original[offset..].find(search_value)
      } else {
        None
      }
    }

    Ok(self)
  }

  fn _split(&mut self, index: u32) -> Result<()> {
    if self.start_index_chunk_map.contains_key(&index)
      || self.end_index_chunk_map.contains_key(&index)
    {
      return Ok(());
    }

    let chunk = self.last_searched_chunk.clone();

    let is_forward = index < chunk.borrow().start;

    let mut cur = Some(chunk);

    while let Some(c) = cur {
      if c.borrow().contains(index) {
        self._split_chunk(c, index)?;
        return Ok(());
      } else {
        cur = {
          if is_forward {
            self
              .end_index_chunk_map
              .get(&c.borrow().start)
              .map(Rc::clone)
          } else {
            self
              .start_index_chunk_map
              .get(&c.borrow().end)
              .map(Rc::clone)
          }
        }
      }
    }

    Ok(())
  }

  fn _split_chunk(&mut self, chunk: Rc<RefCell<Chunk>>, index: u32) -> Result<()> {
    if chunk.borrow().is_edited() && !chunk.borrow().content.is_empty() {
      if let Some((line, column)) = self._locator.locate(index as usize) {
        return Err(Error::from_reason(
          MsErrType::SplitChunk,
          format!(
            "Cannot split a chunk that has already been edited ({}:{} – '{}')",
            line,
            column,
            chunk.borrow().original
          )
          .as_str(),
        ));
      } else {
        return Err(Error::from_reason(
          MsErrType::SplitChunk,
          "Cannot split a chunk that has already been edited",
        ));
      }
    }

    let new_chunk = Chunk::split(Rc::clone(&chunk), index)?;
    self
      .start_index_chunk_map
      .insert(index, Rc::clone(&new_chunk));
    self.end_index_chunk_map.insert(index, Rc::clone(&chunk));
    self
      .end_index_chunk_map
      .insert(new_chunk.borrow().end, Rc::clone(&new_chunk));
    self.last_searched_chunk = Rc::clone(&chunk);
    if self.last_chunk == chunk {
      self.last_chunk = new_chunk
    }
    Ok(())
  }

  pub fn generate_map(&self, options: Option<GenerateMapOptions>) -> Result<SourceMap> {
    let decoded_map = self.generate_decoded_map(options)?;
    SourceMap::from_decoded_map(decoded_map)
  }

  pub fn generate_decoded_map(&self, options: Option<GenerateMapOptions>) -> Result<DecodedMap> {
    let GenerateMapOptions {
      file,
      source,
      hires,
      include_content,
      source_root,
    } = options.unwrap_or_default();

    let hires = hires.unwrap_or_default();

    let mut map = MappingsFacade::new(hires);

    map.advance(self.intro.as_str());

    Chunk::each_next(Rc::clone(&self.first_chunk), |chunk| {
      let loc = self._locator.locate(chunk.borrow().start as usize);
      if let Some((o_line, o_column)) = loc {
        map.add_chunk(
          Rc::clone(&chunk),
          (o_line as u32, o_column as u32),
          self
            .stored_names
            .binary_search(&chunk.borrow().original)
            .unwrap_or_else(|_| usize::MAX),
        );
      }
      Ok(false)
    })?;

    map.advance(self.outro.as_str());

    Ok(DecodedMap {
      version: SOURCEMAP_VERSION,
      file: file
        .as_ref()
        .map(|x| x.split(&['/', '\\'][..]).last().map(String::from))
        .flatten(),
      sources: vec![source
        .as_ref()
        .map(|x| get_relative_path(&file.unwrap_or_default(), x))
        .unwrap_or_default()],
      sources_content: include_content.and_then(|x| {
        if x {
          return Some(vec![self.original.to_owned()]);
        } else {
          return None;
        }
      }),
      source_root,
      names: self.stored_names.to_owned(),
      mappings: map.get_decoded_mappings(),
      x_google_ignoreList: if self.ignore_list {
        Some(vec![SOURCE_INDEX])
      } else {
        None
      },
    })
  }
}

impl ToString for __internal_magic_string {
  fn to_string(&self) -> String {
    // let chunk = self.first_chunk.borrow();
    let mut str = self.intro.clone();
    let mut cur = Some(Rc::clone(&self.first_chunk));
    while let Some(c) = cur {
      let _cur = c.borrow().to_string();
      str.push_str(_cur.as_str());
      cur = c.borrow().next.clone();
    }
    str.push_str(self.outro.as_str());
    str
  }
}
