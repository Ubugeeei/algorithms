// O(n+k)
// bucketというものに分割してinsertion_sortを行う。
// 例として arr: [1, 5, 28, 25, 100, 52, 27, 91, 22, 99]という配列のソートを考える。
// このとき、bucketのサイズは10とする。(サイズの求め方はケースによって違う)
// bucket: [[], [], [], [], [], [], [], [], [], []] -----------------------------※ 1
// arrの各要素を10で割、その商を対応するbucketのインデックスとして振り分ける。 --------※ 2
// この時、bucketは以下のようになる。
// bucket: [[1, 5], [], [28, 25, 27, 22], [], [52], [], [], [], [], [100, 91, 99]]
// bucketの中の要素をinsertion_sortを行う。 ----------------------------------------※ 3
// この時、bucketは以下のようになる。
// bucket: [[1, 5], [], [22, 25, 27, 28], [], [52], [], [], [], [], [91, 99, 100]]
// その後、結合する // --------------------------------------------------------------※ 4
// [1, 5,22, 25, 27, 28, 52, 91, 99, 100]

#[allow(dead_code)]
fn bucket_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let copy = numbers.clone();

    let max = match copy.iter().max() {
        Some(max) => *max,
        None => 0,
    };
    let bucket_size = max / numbers.len() as isize;
    let mut bucket = vec![vec![]; bucket_size as usize]; // ※ 1

    for i in 0..copy.len() {
        // ※ 2
        let index = copy[i] / 10;
        bucket[index as usize].push(copy[i]);
    }

    for i in 0..bucket.len() {
        // ※ 3
        bucket[i] = insertion_sort(&bucket[i]);
    }

    bucket.into_iter().flatten().collect::<Vec<_>>() // ※ 4
}

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
    fn bucket_sort() {
        assert_eq!(
            vec![1, 2, 3, 4, 6, 7],
            insertion_sort(&vec![3, 6, 2, 4, 1, 7])
        )
    }
}
