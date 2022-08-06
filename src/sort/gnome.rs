// indexの要素とindex+1の要素を比較
// indexの要素のほうが大きければ入れ替える。
// ここで、ループのindexをインクリメントせずにディクリメントする。
// 以下同様
// O(n^2)

#[allow(dead_code)]
fn gnome_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();

    let mut index = 0;
    while index < copy.len() {
        if index == 0 || copy[index] >= copy[index - 1] {
            index += 1;
        } else {
            copy.swap(index, index - 1);
            index -= 1;
        }
    }

    copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gnome_sort() {
        assert_eq!(vec![1, 2, 3, 4, 6, 7], gnome_sort(&vec![3, 6, 2, 4, 1, 7]))
    }
}
