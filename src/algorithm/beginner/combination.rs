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
}
