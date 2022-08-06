#[allow(dead_code)]
fn get_divisor(n: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for i in 1..(n + 1) {
        if n % i == 0 {
            res.push(i);
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tes_get_divisor() {
        assert_eq!(get_divisor(1), vec![1]);
        assert_eq!(get_divisor(2), vec![1, 2]);
        assert_eq!(get_divisor(3), vec![1, 3]);
        assert_eq!(get_divisor(4), vec![1, 2, 4]);
        assert_eq!(get_divisor(6), vec![1, 2, 3, 6]);
        assert_eq!(get_divisor(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(get_divisor(15), vec![1, 3, 5, 15]);
        assert_eq!(get_divisor(24), vec![1, 2, 3, 4, 6, 8, 12, 24]);
    }
}
