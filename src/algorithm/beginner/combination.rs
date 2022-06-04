// コンビニにはN個の品物が売られており、i番目（1≦i≦N）の商品の値段はAi円です。異なる2つの品物を買う方法のうち、合計値段が500円となるものは何通りありますか。
// 制約：2≦N≦200000,Aiは100,200,300,400のいずれか

#[allow(dead_code)]
fn solution(item_list: Vec<u32>) -> u32 {
    let mut count_100 = 0;
    let mut count_200 = 0;
    let mut count_300 = 0;
    let mut count_400 = 0;

    for item in item_list {
        if item == 100 {
            count_100 += 1;
        } else if item == 200 {
            count_200 += 1;
        } else if item == 300 {
            count_300 += 1;
        } else if item == 400 {
            count_400 += 1;
        }
    }

    count_100 * count_400 + count_200 * count_300
}

// N枚のカードがあり、左からi番目（1≦i≦N）のカードの色はAiです。Ai=1のとき赤色、Ai=2のとき黄色、Ai=3のとき青色です。同じ色のカードを2枚選ぶ方法は何通りありますか。
#[allow(dead_code)]
fn solution2(item_list: Vec<u32>) -> u32 {
    fn re(c1: u32, c2: u32, c3: u32, c4: u32, i: u32, _item_list: &Vec<u32>) -> u32 {
        if i == _item_list.len() as u32 {
            return c1 * c4 + c2 * c4;
        }

        match _item_list[i as usize] {
            100 => re(c1 + 1, c2, c3, c4, i + 1, _item_list),
            200 => re(c1, c2 + 1, c3, c4, i + 1, _item_list),
            300 => re(c1, c2, c3 + 1, c4, i + 1, _item_list),
            400 => re(c1, c2, c3, c4 + 1, i + 1, _item_list),
            _ => re(c1, c2, c3, c4, i + 1, _item_list),
        }
    }

    re(0, 0, 0, 0, 0, &item_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinatin() {
        assert_eq!(solution(vec![100, 200, 300, 400]), 2);
        assert_eq!(
            solution(vec![100, 200, 300, 400, 200, 300, 200, 200, 100, 400]),
            12
        );
    }

    #[test]
    fn test_combinatin2() {
        assert_eq!(solution2(vec![100, 200, 300, 400]), 2);
        assert_eq!(
            solution(vec![100, 200, 300, 400, 200, 300, 200, 200, 100, 400]),
            12
        );
    }
}
