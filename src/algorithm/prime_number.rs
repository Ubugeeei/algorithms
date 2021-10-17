/**
 * 愚直
 */
#[allow(dead_code)]
fn calc_prime_number(max_range: isize) -> Vec<isize> {
    // パフォーマンス計測のために助産回数をカウント
    let mut div_count = 0;
    let mut prime_numbers: Vec<isize> = Vec::new();

    for num in 1..max_range + 1 {
        let mut divisor_count = 0;

        for i in 1..num + 1 {
            div_count += 1;
            if num % i == 0 {
                divisor_count += 1
            }
        }

        // 約数が自身と1(2つ)の場合に素数と判定する
        if divisor_count == 2 {
            prime_numbers.push(num)
        }
    }
    println!("dix count of 'calc_prime_number': {:?}", div_count);
    prime_numbers
}

/**
 * 改良版
 */
#[allow(dead_code)]
fn calc_prime_number2(max_range: usize) -> Vec<usize> {
    // パフォーマンス計測のために助産回数をカウント
    let mut div_count = 0;
    let mut prime: Vec<usize> = vec![2, 3];
    let mut ptr = 2;

    let mut n: usize = 5;
    while n <= max_range {
        let mut i = 1;
        'inner: while i < ptr {
            div_count += 1;
            if n % prime[i] == 0 {
                break 'inner;
            }
            i += 1;
        }
        if ptr == i {
            ptr += 1;
            prime.push(n);
        }
        n += 2;
    }
    println!("div count: {}", div_count);

    prime
}

/**
 * 改良版2
 */
#[allow(dead_code)]
fn calc_prime_number3(max_range: i32) -> Vec<i32> {
    // パフォーマンス計測のために助産回数をカウント
    let mut div_count = 0;
    let mut prime = vec![2, 3];

    let mut n = 5;
    while n <= max_range {
        let mut flag = false;

        let mut i = 1;
        'inner: while prime[i] * prime[i] <= n {
            div_count += 1;
            if n % prime[i] == 0 {
                flag = true;
                break 'inner;
            }
            i += 1;
        }

        if !flag {
            prime.push(n);
        }
        n += 2;
    }
    println!("div count: {}", div_count);

    prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_prime_number() {
        assert_eq!(calc_prime_number(10), vec![2, 3, 5, 7])
    }

    #[test]
    fn test_calc_prime_number2() {
        assert_eq!(calc_prime_number2(10), vec![2, 3, 5, 7])
    }

    #[test]
    fn test_calc_prime_number3() {
        assert_eq!(calc_prime_number3(10), vec![2, 3, 5, 7])
    }
}
