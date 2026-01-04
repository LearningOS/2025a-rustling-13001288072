pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 测试偶数：调用is_even(4)应该返回true，assert!验证条件为真
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        // 测试奇数：调用is_even(5)应该返回false，assert!验证条件为假（加!取反）
        assert!(!is_even(5));
    }
}
