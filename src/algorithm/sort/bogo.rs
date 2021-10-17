// Bogoソート: テキトーに並び替えるソート
// ただのシャッフル
use rand::seq::SliceRandom;

fn bogo_sort(numbers: Vec<isize>) -> Vec<isize> {
  let mut copy = numbers.clone();
  let mut rng = rand::thread_rng();
  copy.shuffle(&mut rng);
  copy
}

#[cfg(test)]
mod tess {
  use super::*;
  #[test]
  fn test_bogo_sort() {
    let result = bogo_sort(vec![1, 2, 3]);
    println!("{:?}", result);
  }
}