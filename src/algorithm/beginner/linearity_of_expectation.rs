// Linearity of Expectation
// https://brilliant.org/wiki/linearity-of-expectation/#:~:text=Linearity%20of%20expectation%20is%20the,of%20whether%20they%20are%20independent.

// 青・赤2つのN面体サイコロがあります。各サイコロの出目は以下の通りです。
// ・青のサイコロ：B1,B2,...,BNが等確率で出る
// ・赤のサイコロ：R1,R2,...,RNが等確率で出る
// あなたは2つのサイコロを同時に振り、出目の合計だけ賞金がもらえます。
// もらえる賞金の期待値を計算してください。制約：2≦N≦100000,0≦Bi,Ri≦100
#[allow(dead_code)]
fn sol(n: u32) -> f32 {
    let mut result = 0;

    for i in 1..(n + 1) {
        result += 2 * i;
    }

    (result / n) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        assert_eq!(sol(4), 5f32);
    }
}
