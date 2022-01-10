// 櫛の幅を定めて小さくしていく
// 櫛の幅 gap = list len / 1.3
// gap文離れている要素を比較してく
// [2, 9, 1, 4, 7, 3, 5]の場合、
// gap = 7 / 1.3 = 5 なので、
// [2, 9, 1, 4, 7, 3, 5]で2絡みて5分の「3」を比較していく
// 今回は 2<3なので入れ替えない。
// 次は9から見るので、対象化5
// 9>5なので入れ替える。[2, 5, 1, 4, 7, 3, 9]
// 次はギャップを狭める gap = 5 / 1.3 = 3
// 2から
// O(n^2 / 2**g)

#[allow(dead_code)]
fn comb_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();

    let mut gap = copy.len();
    let mut is_swapped = true;

    while is_swapped || gap > 1 {
        if gap > 1 {
            gap = gap * 10 / 13;
        }

        is_swapped = false;

        let mut i = 0;
        while i < copy.len() - gap {
            if copy[i] > copy[i + gap] {
                copy.swap(i, i + gap);
                is_swapped = true;
            }
            i += 1;
        }
    }

    copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb_sort() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7],
            comb_sort(&vec![7, 6, 5, 4, 3, 2, 1])
        );
    }
}
