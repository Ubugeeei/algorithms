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