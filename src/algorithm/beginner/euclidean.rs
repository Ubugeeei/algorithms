#[allow(dead_code)]
fn euclidean(_a: i32, _b: i32) -> i32 {
    let mut a = _a;
    let mut b = _b;

    while a >= 1 && b >= 1 {
        if a < b {
            b = b % a;
        } else {
            a = a % b;
        }
    }

    if a >= 1 {
        return a;
    }

    b
}

#[allow(dead_code)]
fn euclidean_tri(a: i32, b: i32, c: i32) -> i32 {
    let res = euclidean(a, b);
    euclidean(res, c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euclidean() {
        assert_eq!(euclidean(33, 88), 11);
    }

    #[test]
    fn test_euclidean2() {
        assert_eq!(euclidean(123, 777), 3);
    }

    #[test]
    fn test_euclidean_tri() {
        assert_eq!(euclidean_tri(8, 24, 56), 8);
    }
}
