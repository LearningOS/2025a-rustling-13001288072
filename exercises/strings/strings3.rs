fn trim_me(input: &str) -> String {
    // 使用trim()方法去除字符串两端的空白字符，再转换为String
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // 方法1：使用format!宏拼接字符串（推荐，简洁高效）
    format!("{} world!", input)
    
    // 方法2（备选）：先转String再拼接
    // input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // 使用replace()方法替换指定子串，返回新的String
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
