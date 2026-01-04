pub fn bigger(a: i32, b: i32) -> i32 {
    // 通过 if-else 直接比较并返回更大的数值，满足无额外变量/函数调用的要求
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
