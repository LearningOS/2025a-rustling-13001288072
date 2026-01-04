#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 1. 让测试编译并通过：assert_eq! 需要两个相等的参数
        // assert_eq!(42, 42); 

        // 2. 让测试编译但失败：传入两个不相等的参数（取消注释下面这行，注释上面的）
        assert_eq!(42, 13);
    }
}
