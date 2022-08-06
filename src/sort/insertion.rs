// indexとindex+1の要素を比較
// indexの要素のほうが大きければ入れ替える。
// このとき、index+1の要素を適切な位置に入れ替える。
// 以下同様
// O(n^2)

#[allow(dead_code)]
fn insertion_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();

    for i in 0..(copy.len() - 1) {
        let mut j = i;
        while j > 0 && copy[j] < copy[j - 1] {
            copy.swap(j, j - 1);
            j -= 1;
        }
    }
    copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertion_sort() {
        assert_eq!(
            vec![1, 2, 3, 4, 6, 7],
            insertion_sort(&vec![3, 6, 2, 4, 1, 7])
        )
    }
}
