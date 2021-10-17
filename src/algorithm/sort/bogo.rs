// Bogoソート: テキトーに並び替えるソート
// ただのシャッフル
use rand::seq::SliceRandom;

fn bogo_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();
    let mut rng = rand::thread_rng();
    copy.shuffle(&mut rng);
    copy
}

fn in_order(numbers: &Vec<isize>) -> bool {
    let mut copy = numbers.clone();
    for i in 0..copy.len() - 1 {
        if copy[i] > copy[i + 1] { return false }
    }
    true
}

fn bogo_in_order(numbers: Vec<isize>) -> Vec<isize> {
  let copy = numbers.clone();
  let mut sorted = bogo_sort(&copy);
  while !in_order(&sorted) {
    sorted = bogo_sort(&sorted);
  }
  sorted
}

#[cfg(test)]
mod tess {
    use super::*;
    #[test]
    fn test_bogo_sort() {
        let result = bogo_sort(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_ne!(vec![1, 2, 3], result);
    }

    #[test]
    fn test_bogo_in_order() {
        let mut result = bogo_sort(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_ne!(vec![1, 2, 3], result);
        result = bogo_in_order(result);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], result);
    }
}
