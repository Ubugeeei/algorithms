//! linear congruential generator
//! 
#![allow(dead_code)]

const MAX: u32 = 0xffff_ffff;

struct Rand {
  seed: u32,
}

impl Rand {
  fn new(x: u32) -> Rand {
    Rand { seed: x }
  }

  /// return ∈ Int(0, MAX)
  fn rand(&mut self) -> u32 {
    let x = self.seed as u64;
    self.seed = ((69069 * x + 1) & MAX as u64) as u32;
    self.seed
  }

  /// return ∈ R(0, 1)
  fn rand2(&mut self) -> f64 {
    (1.0 / (MAX as f64 + 1.0)) * self.rand() as f64
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rand() {
    let mut r = Rand::new(0);
    for _ in 0..10 {
      assert!(r.rand() < MAX);
    }

    let mut r = Rand::new(MAX);
    for _ in 0..10 {
      assert!(r.rand2() < 1.0 && r.rand2() >= 0.0);
    }
  }
}
