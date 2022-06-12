// a1=0,a2=2
// an=min(an1+|hn1－hn|,an2+|hn2－hn|)(n≧3）
// ただし、数列hの最初の5項は8,6,9,2,1であるとする
#[allow(dead_code)]
fn sol_3_7_3(n: u32) -> u32 {
    fn abs(x: i32) -> u32 {
        if x < 0 {
            (x * -1) as u32
        } else {
            x as u32
        }
    }
    match n {
        1 => 0,
        2 => 2,
        n => {
            let mut sequence = vec![0, 2];
            let sequence_h = vec![8, 6, 9, 2, 1];

            for i in 2..n {
                let an_1 = sequence[(i - 1) as usize];
                let hn_1 = sequence_h[(i - 1) as usize];
                let hn = sequence_h[i as usize];

                let an_2 = sequence[(i - 2) as usize];
                let hn_2 = sequence_h[(i - 2) as usize];

                let a = an_1 + abs(hn_1 - hn);
                let b = an_2 + abs(hn_2 - hn);

                if a > b {
                    sequence.push(b);
                } else {
                    sequence.push(a);
                }
            }
            sequence[(n - 1) as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol_3_7_3() {
        assert_eq!(sol_3_7_3(1), 0);
        assert_eq!(sol_3_7_3(2), 2);
        assert_eq!(sol_3_7_3(3), 1);
        assert_eq!(sol_3_7_3(4), 6);
        assert_eq!(sol_3_7_3(5), 7);
    }
}
