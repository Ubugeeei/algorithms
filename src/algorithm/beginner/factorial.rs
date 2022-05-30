#[allow(dead_code)]
fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }
    n * factorial(n - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn factorial_test() {
        assert_eq!(24, factorial(4));
        assert_eq!(120, factorial(5));
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(479001600, factorial(12));
    }
}
