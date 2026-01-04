// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 处理时间超出24小时的情况
    if time_of_day > 23 {
        None
    } else if time_of_day < 22 { // 22点前（10PM前）有5个冰淇淋
        Some(5)
    } else { // 22点及以后（10PM及以后）没有冰淇淋
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // 提取Option中的值：使用unwrap()获取Some中的值（确保是Some才可用）
        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);
    }
}
