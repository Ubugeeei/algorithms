// n log n

// 例として、arr: [5, 4, 1, 8, 7, 3, 2, 9]のソートを考える

// まず最初に配列を2分割する
// arr: [5, 4, 1, 8,], [ 7, 3, 2, 9]

// これを繰り返し、要素が1つになるまでぶんかつする(ここでは各グループをネストで表現)
// [5, 4, 1, 8, 7, 3, 2, 9]
//    [5, 4, 1, 8,]
//        [5, 4]
//            [5]
//            [4]
//        [1, 8]
//            [1]
//            [8]
//    [7, 3, 2, 9]
//        [7, 3]
//            [7]
//            [3]
//        [2, 9]
//            [2]
//            [9]

// これらを並び替えながらマージしていく
// [5, 4, 1, 8, 7, 3, 2, 9]
//    [5, 4, 1, 8,]
//        [4, 5]
//        [1, 8]
//    [7, 3, 2, 9]
//        [3, 7]
//        [2, 9]

// next
// [5, 4, 1, 8, 7, 3, 2, 9]
//    [1, 4, 5, 8]
//    [2, 3, 7, 9]

// next
// [1, 2, 3, 4, 5, 6, 7, 8, 9]

#[allow(dead_code)]
fn merge_sort(nums: &mut [i32]) {
    let mut sorted = nums.to_vec();

    if nums.len() <= 1 {
        return;
    }

    let center = nums.len() / 2;
    merge_sort(&mut nums[..center]);
    merge_sort(&mut nums[center..]);

    // merge
    merge(&nums[..center], &nums[center..], &mut sorted);

    nums.copy_from_slice(&sorted);
}

#[allow(dead_code)]
fn merge(l_arr: &[i32], r_arr: &[i32], sorted: &mut [i32]) {
    let (mut left, mut right, mut i) = (0, 0, 0);

    while left < l_arr.len() && right < r_arr.len() {
        if l_arr[left] <= r_arr[right] {
            sorted[i] = l_arr[left];
            i += 1;
            left += 1;
        } else {
            sorted[i] = r_arr[right];
            i += 1;
            right += 1;
        }
    }

    if left < l_arr.len() {
        sorted[i..].copy_from_slice(&l_arr[left..]);
    }

    if right < r_arr.len() {
        sorted[i..].copy_from_slice(&r_arr[right..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sort() {
        let mut arr = [7, 4, 6, 3, 5, 2, 1];
        merge_sort(&mut arr);

        assert_eq!([1, 2, 3, 4, 5, 6, 7], arr);
    }
}
