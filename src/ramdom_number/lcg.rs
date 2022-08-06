//! linear congruential generator
//! linear congruence method is a alglithm of generating pseudorandom number
//! Xi = (A * Xi-1 + C) mod M
#![allow(dead_code)]

const M: u32 = 0xffff_ffff;
const A: u64 = 69069;
/// if C = 0, then the algorithm is called multiplicative congruence
const C: u64 = 1;

struct Rand {
  seed: u32,
}

impl Rand {
  fn new(seed: u32) -> Rand {
    Rand { seed }
  }

  /// return ∈ Int(0, M)
  fn rand(&mut self) -> u32 {
    let x = self.seed as u64;
    self.seed = ((A * x + C) & M as u64) as u32;
    self.seed
  }

  /// return ∈ R(0, 1)
  fn rand2(&mut self) -> f64 {
    (1.0 / (M as f64 + 1.0)) * self.rand() as f64
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rand() {
    let mut r = Rand::new(0);
    for _ in 0..10 {
      assert!(r.rand() < M);
    }

    let mut r = Rand::new(M);
    for _ in 0..10 {
      assert!(r.rand2() < 1.0 && r.rand2() >= 0.0);
    }
  }
}
