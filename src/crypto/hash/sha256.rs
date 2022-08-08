//! The SHA256 hash algorithm.

#![allow(dead_code)]

#[derive(Copy, Clone)]
pub struct SHA256;

impl SHA256 {
  /**
   * constant
   */
  const BLOCK_SIZE: usize = 64;
  const DELIMITER: u32 = 0x80;
  /// 4bytes after the decimal point of the cube root of 64 prime numbers from smallest to largest
  const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xD6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
  ];
  /// 4bytes after the decimal point of the square root of 8 prime numbers from smallest to largest
  const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
  ];

  /**
   * constructor
   */
  pub fn new() -> Self {
    SHA256
  }

  /// calculate the hash of the message
  pub fn hash(self, message: &Vec<u8>) -> Vec<u32> {
    let message: Vec<u32> = message.iter().map(|n| *n as u32).collect();
    let n = message.len() / SHA256::BLOCK_SIZE;
    let mut w = vec![0; SHA256::BLOCK_SIZE];
    let mut h = vec![];

    for (i, it) in SHA256::H.iter().enumerate() {
      h[i] = *it;
    }

    for i in 1..n {
      for t in 0..SHA256::BLOCK_SIZE {
        // TODO:
        w[t] = if t < 16 {
          let p = (i - 1) * SHA256::BLOCK_SIZE + t * 4;
          (message[p] << 24)
            .wrapping_add(message[p + 1] << 16)
            .wrapping_add(message[p + 2] << 8)
            .wrapping_add(message[p + 3])
        } else {
          (self
            .gamma1(w[t - 2])
            .wrapping_add(w[t - 7])
            .wrapping_add(self.gamma0(w[t - 15]))
            .wrapping_add(w[t - 16]))
            & 0xffffffff
        };
      }

      let mut a = h[0];
      let mut b = h[1];
      let mut c = h[2];
      let mut d = h[3];
      let mut e = h[4];
      let mut f = h[5];
      let mut g = h[6];
      let mut hh = h[7];

      for t in 0..SHA256::BLOCK_SIZE {
        // TODO:
        let t1 = hh + self.sigma1(e) + self.ch(e, f, g) + SHA256::K[t] + w[t] & 0xffffffff;
        let t2 = self.sigma0(a) + self.maj(a, b, c) & 0xffffffff;
        hh = g;
        g = f;
        f = e;
        e = (d + t1) & 0xffffffff;
        d = c;
        c = b;
        b = a;
        a = (t1 + t2) & 0xffffffff;
      }
    }

    h
  }

  /**
   * funuctions
   */
  fn rotr(self, x: u32, n: u32) -> u32 {
    (x >> n) | (x << (32 - n))
  }
  fn shr(self, x: u32, n: u32) -> u32 {
    x >> n
  }
  fn ch(self, x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
  }
  fn maj(self, x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
  }
  fn sigma0(self, x: u32) -> u32 {
    self.rotr(x, 2) ^ self.rotr(x, 13) ^ self.rotr(x, 22)
  }
  fn sigma1(self, x: u32) -> u32 {
    self.rotr(x, 6) ^ self.rotr(x, 11) ^ self.rotr(x, 25)
  }
  fn gamma0(self, x: u32) -> u32 {
    self.rotr(x, 7) ^ self.rotr(x, 18) ^ self.shr(x, 3)
  }
  fn gamma1(self, x: u32) -> u32 {
    self.rotr(x, 17) ^ self.rotr(x, 19) ^ self.shr(x, 10)
  }

  /// add padding and sizes to the message
  /// returns: 64 bytes as a u8 array
  pub fn add_padding(self, message: &Vec<u8>) -> Vec<u32> {
    const SIZE_BYTES: usize = 8;

    let len = message.len();

    let mut tmp = vec![0x00u32; SHA256::BLOCK_SIZE];
    tmp[0] = SHA256::DELIMITER;

    // add padding
    let mut padded = message.iter().map(|n| *n as u32).collect();
    padded = if len % SHA256::BLOCK_SIZE < SHA256::BLOCK_SIZE - SIZE_BYTES {
      vec![
        padded,
        tmp[0..(SHA256::BLOCK_SIZE - SIZE_BYTES - len % SHA256::BLOCK_SIZE)].to_vec(),
      ]
      .concat()
    } else {
      vec![
        padded,
        tmp[0..(SHA256::BLOCK_SIZE + SHA256::BLOCK_SIZE - SIZE_BYTES - len % SHA256::BLOCK_SIZE)]
          .to_vec(),
      ]
      .concat()
    };

    // add length
    let len_bits = (len * 8) as u64;
    let mut size = vec![0x00; 8];
    size[4] = (len_bits >> 24) as u32;
    size[5] = (len_bits >> 16) as u32;
    size[6] = (len_bits >> 8) as u32;
    size[7] = (len_bits >> 0) as u32;

    vec![padded, size].concat()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sha256_padding() {
    {
      let sha256 = SHA256::new();
      let pdd = sha256.add_padding(&vec![]);
      assert_eq!(pdd.len(), 64);
      assert_eq!(pdd[0], 0x80);
    }
    {
      let sha256 = SHA256::new();
      let pdd = sha256.add_padding(&vec![1, 2, 3, 4, 5, 6, 7, 8]);
      assert_eq!(pdd.len(), 64);
      assert_eq!(pdd[0], 1);
      assert_eq!(pdd[8], 0x80);
    }
  }
}
