#[allow(dead_code)]
fn divide_and_conquer(v: &Vec<u32>, l: u32, r: u32) -> u32 {
    if r - l == 1 {
        v[l as usize]
    } else {
        let m: u32 = (l + r) / 2;
        let s1 = divide_and_conquer(&v, l, m);
        let s2 = divide_and_conquer(&v, m, r);

        s1 + s2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_divide_and_conquer() {
        let v = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        assert_eq!(divide_and_conquer(&v, 0, 10), 55);
        assert_eq!(divide_and_conquer(&v, 0, v.len() as u32), 210);
    }
}
