#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        // 1. 先让测试编译并通过：assert! 接收一个布尔值，传入true即可
        // assert!(true); 

        // 2. 让测试失败：传入false（取消注释下面这行，注释上面的true）
        assert!(false);
    }
}
