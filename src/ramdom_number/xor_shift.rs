#![allow(dead_code)]

struct XorShift {
  value: u128,
}

impl XorShift {
  fn new(seed: u128) -> XorShift {
    XorShift { value: seed }
  }

  fn next(&mut self) -> u128 {
    self.value = self.value ^ (self.value << 13);
    self.value = self.value ^ (self.value >> 17);
    self.value = self.value ^ (self.value << 5);
    self.value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_xor_shift() {
    let mut r = XorShift::new(2463534242);
    for _ in 0..10 {
      println!("{} ", r.next());
    }

    assert!(true)
  }
}
