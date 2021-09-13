pub fn calc_prime_number(max_range: isize) {
  let mut prime_numbers = vec![];

  for num in 1..max_range+1 {
    let mut divisor_count = 0;

    for i in 1..num+1 {
      if num % i == 0 { divisor_count += 1 }
    }

    // 約数が自身と1(2つ)の場合に素数と判定する
    if divisor_count == 2 { prime_numbers.push(num) }
  }
  dbg!(prime_numbers);
}

pub fn calc_prime_number2(max_range: i32) {
  let mut prime = vec![2, 3];
  let mut ptr = 2;

  let mut n = 5;
  while n <= max_range {

    let mut i = 1;
    'inner: while i < ptr  {
      if n % prime[i]  == 0 { break 'inner; }
      i += 1;
    }
    if ptr == i {
      ptr += 1;
      prime.push(n);
    }
    n += 2;
  }
  dbg!(prime);
}