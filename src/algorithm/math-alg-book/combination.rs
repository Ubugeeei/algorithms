// ex1)
// コンビニにはN個の品物が売られており、i番目（1≦i≦N）の商品の値段はAi円です。異なる2つの品物を買う方法のうち、合計値段が500円となるものは何通りありますか。
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

#[allow(dead_code)]
fn solution2(item_list: Vec<u32>) -> u32 {
    fn re(c1: u32, c2: u32, c3: u32, c4: u32, i: u32, _item_list: &Vec<u32>) -> u32 {
        if i == _item_list.len() as u32 {
            return c1 * c4 + c2 * c3;
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

// ex2)
// N枚のカードがあり、左からi番目（1≦i≦N）のカードの色はAiです。Ai=1のとき赤色、Ai=2のとき黄色、Ai=3のとき青色です。同じ色のカードを2枚選ぶ方法は何通りありますか。
#[allow(dead_code)]
fn solution3(cards: Vec<u32>) -> u32 {
    let (mut red_count, mut yellow_count, mut blue_count) = (0, 0, 0);

    fn c(_all: u32, _i: u32) -> u32 {
        let mut all = _all;
        let mut i = _i;

        let mut deno = i;
        let mut nume = all;

        loop {
            i -= 1;
            all -= 1;
            if i == 0 {
                break;
            }
            deno = deno * i;
            nume = nume * all
        }

        nume / deno
    }

    for card in cards.iter() {
        match card {
            1 => {
                red_count += 1;
            }
            2 => {
                yellow_count += 1;
            }
            3 => {
                blue_count += 1;
            }
            _ => {}
        }
    }

    c(red_count, 2) + c(yellow_count, 2) + c(blue_count, 2)
}

// ex3)
// カードには整数Aiが書かれています。カードを5枚選ぶ方法のうち、選んだカードに書かれた整数の和がちょうど1000となるものは何通りありますか。
#[allow(dead_code)]
fn solution4(cards: Vec<u32>) -> u32 {
    let mut count = 0;
    for card in cards.iter() {
        for _card in cards.iter() {
            for __card in cards.iter() {
                for ___card in cards.iter() {
                    for ____card in cards.iter() {
                        if card + _card + __card + ___card + ____card == 1000 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
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

    #[test]
    fn test_combinatin3() {
        assert_eq!(solution3(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(solution(vec![1, 2, 3]), 0);
    }
}
