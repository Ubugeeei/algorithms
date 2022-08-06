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

#[allow(dead_code)]
fn euclidean_multi(a: &[i32]) -> i32 {
    let mut idx = 0;
    let mut b = a[idx];
    loop {
        if idx == a.len() - 1 {
            break b;
        }
        b = euclidean(b, a[idx + 1]);
        idx += 1;
    }
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

    #[test]
    fn test_euclidean_multi() {
        assert_eq!(euclidean_multi(&[8, 24, 56]), 8);
    }
}
