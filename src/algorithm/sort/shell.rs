// gapを用いる
// オーダーgapに依存する
// 例として arr: [5, 6, 9, 2, 3]のソートを考える
// gap = n / 2とする
// ⅰ. gap = 5/2 = 2であり、5 > 9なので入れ替えは行われない
// ⅱ. 6 > 2なので入れ替えは行われる arr: [5, 2, 9, 6, 3]
// ⅲ. 9と3の比較になるが、3は最後の要素なので、tempに置く
// ⅳ. 9 > 3なので入れ替えは行われる arr: [5, 2, temp, 6, 9], temp=3
// ⅴ. 5 > temp入れ替えは行われる arr: [3, 2, 5, 6, 9]
// ⅵ: ⅰに戻る

#[allow(dead_code)]
fn shell_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();

    let mut gap = copy.len() / 2;

    while gap > 0 {
        for i in gap..copy.len() {
            let temp = numbers[i];
            let mut j = i;
            while j >= gap && copy[j - gap] > temp {
                copy.swap(j, j - gap);
                j -= gap;
            }
            copy[j] = temp;
        }
        gap /= 2;
    }
    copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shell_sort() {
        assert_eq!(vec![1, 2, 3, 4, 6, 7], shell_sort(&vec![3, 6, 2, 4, 1, 7]))
    }
}
