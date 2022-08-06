#[allow(dead_code)]
fn is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tets {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
    }
}
