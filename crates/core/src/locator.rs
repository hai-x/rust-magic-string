pub struct Locator {
  line_offsets: Vec<usize>,
}

impl Locator {
  pub fn new(source: &str) -> Self {
    let original_lines = source.lines();
    let mut line_offsets = Vec::new();
    let mut pos = 0;

    for line in original_lines {
      line_offsets.push(pos);
      pos += line.chars().count() + 1;
    }

    Locator { line_offsets }
  }

  pub fn locate(&self, index: usize) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = self.line_offsets.len();

    while i < j {
      let m = (i + j) >> 1;
      if index < self.line_offsets[m] {
        j = m;
      } else {
        i = m + 1;
      }
    }

    if i == 0 {
      return None; // index is out of bounds
    }

    let line = i - 1;
    let column = index - self.line_offsets[line];
    Some((line, column))
  }
}
