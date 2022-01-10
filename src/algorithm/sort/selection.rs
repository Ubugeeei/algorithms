// index = 0
// index = 0 の要素を保存
// 各要素と比較して、保存した要素より小さければその要素を保存する
// これを繰り替え時、すべての要素と比較したら保存されている要素とindex - 0の要素を入れ替える
// index = 2へ、、以下同様
// O(n^2)る
#[allow(dead_code)]
fn selection_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();

    for i in 0..(copy.len() - 1) {
        let mut min_idx = i;
        for j in (i + 1)..copy.len() - 1 {
            if copy[min_idx] > copy[j] {
                min_idx = j;
            }
        }

        if i != min_idx {
            copy.swap(i, min_idx);
        }
    }

    copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_selection_sort() {
        assert_eq!(
            vec![1, 2, 3, 4, 6, 7],
            selection_sort(&vec![3, 6, 2, 4, 1, 7])
        )
    }
}
