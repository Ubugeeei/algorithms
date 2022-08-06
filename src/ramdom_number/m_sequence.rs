//! Maximum length sequence
//! Xi = C1Xi-1 XOR C2Xi-2 XOR ... XOR CjXi-j
//! Ci ∈ { 0, 1 }
//!

//! simply,
//! Xi = X(i-p) XOR X(i - q)
//! p > q > 0

#![allow(dead_code)]

struct MSequence {
  value: u32,
  p: u32,
  q: u32,
  mask: u32,
}

impl MSequence {
  fn new(seed: u32, p: u32, q: u32) -> MSequence {
    if q > p {
      panic!("q must be greater than p");
    }

    let mask = 2u32.pow(p) - 1;

    MSequence {
      value: seed & mask,
      p,
      q,
      mask,
    }
  }

  fn next(&mut self) -> u32 {
    // let x = self.value;
    // self.value = ((x >> 1) ^ x) as u32;
    // self.value
    let b = ((self.value >> (self.p - 1)) ^ (self.value >> (self.q - 1))) & 1;
    self.value = ((self.value << 1) | b) & self.mask;
    self.value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_m_sequence() {
    let mut r = MSequence::new(2463534242, 4, 1);
    for _ in 0..10 {
      println!("m_sequence: {:#010b} ", r.next());
    }

    assert!(true)
  }
}
