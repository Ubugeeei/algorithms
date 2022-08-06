/// カクテルソート(シェイカーソート)
/// swap(入れ替えたかどうか)という概念を扱う
/// swapの初期値はfalse
/// 先頭から見ていき、比較し、今の値が次の値より大きければ入れ替え、swapをtrueに。
/// 終端まで一周したらswapをfalseに戻す。次は終端から戦闘に向けてループ。
/// 先頭まで行ったらまた先頭からループ。というのを繰り返し、
/// swapがずっとfalseだった時点で処理を終了する
// O(n^2)
#[allow(dead_code)]
fn cocktail_sort(numbers: &Vec<isize>) -> Vec<isize> {
    let mut copy = numbers.clone();
    let len = copy.len();
    // whileで使うので初期はtrueにしておく
    let mut swapped = true;
    let mut start = 0;
    let mut end = len - 1;

    while swapped {
        swapped = false; // init

        // 戦闘からループ
        for i in start..end {
            if copy[i] > copy[i + 1] {
                let _i = copy[i + 1];
                let _i_next = copy[i];
                copy[i] = _i;
                copy[i + 1] = _i_next;
                swapped = true;
            }
        }

        // この時点で入れ替わりが発生してなければソート終了
        if !swapped {
            return copy;
        }

        // 逆順にループ
        start = start + 1;
        end = end - 1;
        loop {
            start -= 1;
            if start < end {
                break;
            }
            let i = start;
            if copy[i] > copy[i + 1] {
                let _i = copy[i + 1];
                let _i_next = copy[i];
                copy[i] = _i;
                copy[i + 1] = _i_next;
                swapped = true;
            }
        }

        if !swapped {
            return copy;
        }
    }
    copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cocktail_sort() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7],
            cocktail_sort(&vec![3, 6, 4, 2, 1, 7, 5])
        )
    }
}
