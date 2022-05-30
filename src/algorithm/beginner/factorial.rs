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

// type Y1<T> = fn(x:T) -> T;
// type Y2<T> = fn(f: Y1<T>, x:T) -> T;
// fn YFactory<T>(f:Y2<T>) -> Y1<T> {
// let mut closure1: Y1<T>;

// let closure2: fn(ff: Y2<T> , xx: T)-> T = |ff: Y2<T> , xx: T| {
// ff(closure1, xx)
// };

// *closure1 = |x| {
// let result = closure2(f, x);
// closure1 = |_| { () }; // Fix memory leak
// result
// };

// closure1
// }

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

    // fn factorial_Y_test() {
    // let fac : Y2<i32> = |f:Y1<i32>, x:i32| {
    // if x==0 { 1 } else { f(x-1) * x }
    // };
    // assert_eq!(120, YFactory(fac)(5));
    // assert_eq!(1, YFactory(fac)(0));
    // assert_eq!(479001600, YFactory(fac)(12));
    // }
}
