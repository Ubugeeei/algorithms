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

// ある国語のテストの問題はN問からなり、すべて選択式問題です。
// i問目(1≦i≦N)はPi個の選択肢から1つの正解を選ぶ形式であり、配点はQi点です。
// 太郎君はまったく手がかりがつかめなかったので、全部の問題をランダムに解答することにしました。
// 太郎君が得られる点数の期待値を計算してください。
// 1 ≦ N ≦ 50,    2 ≦ Pi ≦ 9,    1 ≦ Qi ≦ 200
#[allow(dead_code)]
fn sol3_4_6(n: u32, pi: u32, qi: u32) -> f32 {
    if n < 1 || n > 50 || pi < 2 || pi > 9 || qi < 1 || qi > 200 {
        panic!("range error!");
    } else {
        return ((qi / pi) * n) as f32;
    }
}

// 次郎君の夏休みはN日間あります。彼はi日目(1≦i≦N)の勉強時間を以下の手順で決めます。
// ・1日の最初にサイコロを振る
// ・サイコロを振って1,2が出た場合：Ai時間勉強する
// ・サイコロを振って3,4,5,6が出た場合：Bi時間勉強する
// 彼の夏休みの合計勉強時間の期待値を求めるプログラムを作成してください。
//
#[allow(dead_code)]
fn sol3_4_3(n: u32, ai: u32, bi: u32) -> u32 {
    (ai + 2 * bi) * n / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol() {
        assert_eq!(sol(4), 5f32);
    }

    #[test]
    fn test_sol3_4_6() {
        assert_eq!(sol3_4_6(3, 4, 100), 75f32);
    }

    #[test]
    fn test_sol3_4_3() {
        assert_eq!(sol3_4_3(30, 2, 3), 80);
    }
}
