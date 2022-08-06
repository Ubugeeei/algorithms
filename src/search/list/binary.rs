#[allow(dead_code)]
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
  let mut left = 0;
  let mut right = arr.len() - 1;

  while left <= right {
    let mid = (left + right) / 2;
    if arr[mid] == target {
      return Some(mid);
    }

    if arr[mid] < target {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_binary_search() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(binary_search(&arr, 1), Some(0));
    assert_eq!(binary_search(&arr, 2), Some(1));
    assert_eq!(binary_search(&arr, 3), Some(2));
    assert_eq!(binary_search(&arr, 4), Some(3));
    assert_eq!(binary_search(&arr, 5), Some(4));
    assert_eq!(binary_search(&arr, 6), Some(5));
    assert_eq!(binary_search(&arr, 7), Some(6));
    assert_eq!(binary_search(&arr, 8), Some(7));
    assert_eq!(binary_search(&arr, 9), Some(8));
    assert_eq!(binary_search(&arr, 10), Some(9));
    assert_eq!(binary_search(&arr, 11), None);
  }
}
