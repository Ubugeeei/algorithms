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

// N個の足場があり、左からi番目の足場（足場iとする）の高さはhiです。
// カエルは以下の行動を繰り返すことで、足場1から足場Nに移動したいです。
// ・足場iからi+1にジャンプする。体力|hi－hi+1|を消費する（1≦i≦N－1)
// ・足場iからi+2にジャンプする。体力|hi－hi+2|を消費する（1≦i≦N－2）
// 消費する体力の合計として考えられる最小値を求めてください。
// たとえばN=5、(h1,h2,h3,h4,h5)=(8,6,9,2,1)の場合、3.7.5項と同一の問題となり、答えは7です。
// 制約：2≦N≦100000,1≦hi≦10000
#[allow(dead_code)]
fn sol_dp_1(steps: Vec<i32>) -> i32 {
    fn abs(x: i32) -> i32 {
        if x < 0 {
            x * -1
        } else {
            x
        }
    }

    let mut dp = vec![];
    for i in 0..steps.len() {
        match i {
            0 => {
                dp.push(0);
            }
            1 => {
                let v1 = abs(steps[(i - 1 as usize)] - steps[i as usize]);
                dp.push(v1);
            }
            _ => {
                let v1 = dp[(i - 1) as usize] + abs(steps[(i - 1 as usize)] - steps[i as usize]);
                let v2 = dp[(i - 2) as usize] + abs(steps[(i - 2 as usize)] - steps[i as usize]);

                if v1 > v2 {
                    dp.push(v2);
                } else {
                    dp.push(v1);
                }
            }
        }
    }

    dp[(dp.len() - 1) as usize]
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

    #[test]
    fn test_dp() {
        assert_eq!(sol_dp_1(vec![8, 6, 9, 2, 1]), 7);
    }
}
