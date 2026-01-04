// 使用泛型 T 替代具体的 u32 类型，让 Wrapper 支持任意类型
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    // 构造函数也使用泛型 T，接收任意类型的 value 参数
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
