// O(n)
// 例として、arr:[4, 3, 6, 2, 3, 4, 7]のソートを考える。
// 長さが最大値で、要素が0の配列を作る、今回は最大値が7なので、counts:[0, 0, 0, 0, 0, 0, 0, 0]
// いくつ入っているかカウントする counts:[0, 0, 1, 2, 2, 0, 1, 1]
// indexの要素として、indexの数とindex-1を足したものをとる count: [0, 0, 1, 3, 5, 5, 6, 7]

#[allow(dead_code)]
fn counting_sort(nums: &Vec<isize>) -> Vec<isize> {
    let copy = nums.clone();

    let max = match copy.iter().max() {
        Some(max) => *max,
        None => 0,
    };
    let mut counts = vec![0; max as usize + 1];
    let mut sorted = vec![0; copy.len()];

    for i in 0..copy.len() {
        counts[copy[i as usize] as usize] += 1;
    }

    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }

    for num in copy {
        sorted[counts[num as usize] - 1] = num;
        counts[num as usize] -= 1;
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_counting_sort() {
        assert_eq!(
            vec![1, 2, 3, 4, 6, 7],
            counting_sort(&vec![3, 6, 2, 4, 1, 7])
        )
    }
}
