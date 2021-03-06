// Bogoソート: テキトーに並び替えるソート
// ただのシャッフル
// O((n +1)!)
use rand::seq::SliceRandom;

#[allow(dead_code)]
fn bogo_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();
    let mut rng = rand::thread_rng();
    copy.shuffle(&mut rng);
    copy
}

#[allow(dead_code)]
fn in_order(numbers: &Vec<isize>) -> bool {
    let copy = numbers.clone();
    for i in 0..copy.len() - 1 {
        if copy[i] > copy[i + 1] {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
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
        assert_ne!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], result);
    }

    #[test]
    fn test_bogo_in_order() {
        let mut result = bogo_sort(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_ne!(vec![1, 2, 3, 5, 6, 7, 8, 9], result);
        result = bogo_in_order(result);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], result);
    }
}
