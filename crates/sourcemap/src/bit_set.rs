#[derive(Clone)]
pub struct BitSet {
  bits: Vec<u32>,
}

impl BitSet {
  pub fn new(arg: Option<&BitSet>) -> Self {
    match arg {
      Some(bs) => BitSet {
        bits: bs.bits.clone(),
      },
      None => BitSet { bits: Vec::new() },
    }
  }

  pub fn add(&mut self, n: usize) {
    let index = n >> 5; // n / 32
    if index >= self.bits.len() {
      self.bits.resize(index + 1, 0);
    }
    self.bits[index] |= 1 << (n & 31); // n % 32
  }

  pub fn has(&self, n: usize) -> bool {
    let index = n >> 5; // n / 32
    if index < self.bits.len() {
      (self.bits[index] & (1 << (n & 31))) != 0 // n % 32
    } else {
      false
    }
  }
}
