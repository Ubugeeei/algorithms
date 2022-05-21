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
    }

    #[test]
    fn test_is_prime2() {
        assert_eq!(is_prime(2), true);
    }

    #[test]
    fn test_is_prime3() {
        assert_eq!(is_prime(3), true);
    }

    #[test]
    fn test_is_prime4() {
        assert_eq!(is_prime(4), false);
    }

    #[test]
    fn test_is_prime5() {
        assert_eq!(is_prime(5), true);
    }

    #[test]
    fn test_is_prime6() {
        assert_eq!(is_prime(6), false);
    }
}
