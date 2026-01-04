use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // 初始化空的 HashMap
    let mut basket = HashMap::new();

    // 已有的香蕉
    basket.insert(String::from("banana"), 2);

    // 添加苹果（数量2）、芒果（数量2），满足至少3种水果、总数≥5的要求
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
