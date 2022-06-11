use rand::Rng;

#[allow(dead_code)]
fn get_pi(trials: u64) -> f64 {
    let mut m: f64 = 0.0;

    for _ in 0..trials {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        if x.powi(2) + y.powi(2) <= 1.0 {
            m += 1.0;
        }
    }

    (4f64 * m / trials as f64) as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_pi() {
        let res = get_pi(1000000);
        dbg!(res);
        assert_eq!(res < 4.0, true);
        assert_eq!(res > 3.0, true);
    }
}
