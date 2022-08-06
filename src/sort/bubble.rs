// O(n^2)
#[allow(dead_code)]
fn bubble_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();
    let len = numbers.len();

    for i in 0..len {
        // iを引くことでlimitが上がっていく
        for j in 0..len - 1 - i {
            if copy[j] > copy[j + 1] {
                let _j = copy[j + 1];
                let _j_next = copy[j];
                copy[j] = _j;
                copy[j + 1] = _j_next;
            }
        }
    }

    copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        assert_eq!(vec![1, 2, 3, 4, 6, 7], bubble_sort(&vec![3, 6, 2, 4, 1, 7]))
    }
}
