use std::{cell::RefCell, collections::HashMap, rc::Rc};

use napi::Error;

mod chunk;
use chunk::Chunk;

use crate::error::MagicStringError;

use crate::utils::rx_new;

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
}

impl __internal_magic_string {
  pub fn new(str: &str) -> Self {
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
    }
  }

  pub fn append(&mut self, str: &str) -> Result<&mut Self, Error> {
    self.outro = format!("{}{}", self.outro, str);
    Ok(self)
  }

  pub fn prepend(&mut self, str: &str) -> Result<&mut Self, Error> {
    self.intro = format!("{}{}", str, self.intro);
    Ok(self)
  }

  pub fn append_left(&mut self, index: u32, content: &str) -> Result<&mut Self, Error> {
    self._split(index)?;
    if let Some(chunk) = self.end_index_chunk_map.get(&index) {
      let mut chunk = chunk.borrow_mut();
      chunk.append_left(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn append_right(&mut self, index: u32, content: &str) -> Result<&mut Self, Error> {
    self._split(index)?;
    if let Some(chunk) = self.start_index_chunk_map.get(&index) {
      let mut chunk: std::cell::RefMut<'_, Chunk> = chunk.borrow_mut();
      chunk.append_right(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn prepend_left(&mut self, index: u32, content: &str) -> Result<&mut Self, Error> {
    self._split(index)?;
    if let Some(chunk) = self.end_index_chunk_map.get(&index) {
      let mut chunk: std::cell::RefMut<'_, Chunk> = chunk.borrow_mut();
      chunk.prepend_left(content);
    } else {
      self.intro.push_str(content);
    };
    Ok(self)
  }

  pub fn prepend_right(&mut self, index: u32, content: &str) -> Result<&mut Self, Error> {
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
    let pattern = "^".to_owned() + char_type.unwrap_or("\\s") + "+";
    let reg = rx_new(pattern);
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
    let pattern = char_type.unwrap_or("\\s").to_owned() + "+$";
    let reg = rx_new(pattern);
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
      cur = _cur.previous.clone();
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

  pub fn _move(&mut self, start: u32, end: u32, index: u32) -> Result<&mut Self, Error> {
    if index >= start && index <= end {
      return Err(Error::from(MagicStringError::MoveSelectionError));
    }

    if start > end {
      return Err(Error::from(MagicStringError::MoveStartLargerError));
    }

    self._split(start)?;
    self._split(end)?;
    self._split(index)?;

    let first = self
      .start_index_chunk_map
      .get(&start)
      .map(Rc::clone)
      .unwrap();
    let last = self.end_index_chunk_map.get(&end).map(Rc::clone).unwrap();

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

  fn _split(&mut self, index: u32) -> Result<(), Error> {
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

  fn _split_chunk(&mut self, chunk: Rc<RefCell<Chunk>>, index: u32) -> Result<(), Error> {
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
