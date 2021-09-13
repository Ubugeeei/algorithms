mod algorithm;
fn main() {
    println!("＝＝＝Algorithms＝＝＝");
    println!("prime number");
    algorithm::prime_number::calc_prime_number(100);
    algorithm::prime_number::calc_prime_number2(100);
    algorithm::prime_number::calc_prime_number3(100);
}