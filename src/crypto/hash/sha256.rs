//! The SHA256 hash algorithm.

#![allow(dead_code)]

pub struct SHA256;

impl SHA256 {
  /**
   * constant
   */
  const BLOCK_SIZE: usize = 64;
  const DELIMITER: u8 = 0x80;
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

  /**
   * constructor
   */
  pub fn new() -> Self {
    SHA256
  }

  /// add padding and sizes to the message
  /// returns: 64 bytes as a u8 array
  pub fn add_padding(self, message: &Vec<u8>) -> Vec<u8> {
    const SIZE_BYTES: usize = 8;

    let len = message.len();

    let mut tmp = vec![0x00; SHA256::BLOCK_SIZE];
    tmp[0] = SHA256::DELIMITER;

    // add padding
    let mut padded = message.clone();
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
    size[4] = (len_bits >> 24) as u8;
    size[5] = (len_bits >> 16) as u8;
    size[6] = (len_bits >> 8) as u8;
    size[7] = (len_bits >> 0) as u8;

    vec![padded, size].concat()
  }

  /**
   * utils
   */
  fn rotr(self, x: u32, n: u32) -> u32 {
    (x >> n) | (x << (32 - n))
  }
  fn shr(self, x: u32, n: u32) -> u32 {
    x >> n
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
