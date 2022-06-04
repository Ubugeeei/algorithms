// 基数ソート
// O(n)
// counting sortを利用する
// arr: [24, 10, 11, 324, 201, 101, 55]のソートを考える
// まずは1の位を見て並び替える [10, 11, 201, 101, 24, 324. 55]
// つぎに10の位を見て並び替える [201, 101, 10, 11, 24, 324. 55]
// つぎに100の位を見て並び替える [10, 11, 24, 55, 101, 201, 324]
// つぎに・・・・

#[allow(dead_code)]
fn radix_sort(nums: &Vec<isize>) -> Vec<isize> {
    let mut copy = nums.clone();

    let max = match copy.iter().max() {
        Some(max) => *max,
        None => 0,
    };

    let mut place = 1;

    while max > place {
        copy = counting_sort(&copy, place);
        place *= 10;
    }

    copy
}

#[allow(dead_code)]
fn counting_sort(nums: &Vec<isize>, place: isize) -> Vec<isize> {
    let copy = nums.clone();

    let mut counts = vec![0; 10];
    let mut sorted = vec![0; copy.len()];

    for num in copy.iter() {
        let index = (num / place).wrapping_div_euclid(10) as usize;
        if index < copy.len() {
            counts[index] += 1;
        }
    }

    for i in 0..copy.len() {
        counts[i] += counts[i - 1];
    }

    let mut i = copy.len() - 1;
    #[allow(unused_comparisons)]
    while i >= 0 {
        let index = (copy[i] / place).wrapping_div_euclid(10);
        sorted[counts[index as usize] - 1] = copy[i];
        counts[index as usize] -= 1;
        i -= 1;
    }

    sorted
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_radix_sort() {
//         assert_eq!(
//             vec![10, 11, 24, 55, 101, 201, 324],
//             radix_sort(&vec![24, 10, 11, 324, 201, 101, 55])
//         )
//     }
// }
