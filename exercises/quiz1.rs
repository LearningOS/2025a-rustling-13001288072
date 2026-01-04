// 计算苹果总价的函数：
// - 购买数量 ≤40：每个2 rustbucks
// - 购买数量 >40：每个1 rustbuck
fn calculate_price_of_apples(number: i32) -> i32 {
    if number > 40 {
        number * 1 // 超过40个，单价1 rustbuck
    } else {
        number * 2 // 40个及以下，单价2 rustbucks
    }
}

// 无需修改的测试函数
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
