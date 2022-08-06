// O(n log n)

#[allow(dead_code)]
fn partition(nums: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    let pivot = nums[high as usize];
    let mut i = low;

    for j in low..high {
        if nums[j as usize] <= pivot {
            nums.swap(i as usize, j as usize);
            i += 1;
        }
    }

    nums.swap(i as usize, high as usize);

    i
}

#[allow(dead_code)]
fn quick_sqrt(nums: &Vec<i32>) -> Vec<i32> {
    let mut copy = nums.clone();
    let len = copy.len() as i32;

    fn _quick_sqrt(nums: &mut Vec<i32>, low: i32, high: i32) {
        if low < high {
            let partition_index = partition(nums, low, high);
            _quick_sqrt(nums, low, partition_index - 1);
            _quick_sqrt(nums, partition_index + 1, high);
        }
    }

    _quick_sqrt(&mut copy, 0, len - 1);

    copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quick_sqrt() {
        assert_eq!(vec![1, 2, 3, 4, 6, 7], quick_sqrt(&vec![7, 4, 6, 3, 2, 1]));
    }
}