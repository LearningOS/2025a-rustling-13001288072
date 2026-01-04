// Step 1.
// 实现首字母大写功能：取出第一个字符转大写，拼接剩余字符
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            // 首字母转大写 + 剩余字符原样拼接
            first.to_uppercase().to_string() + c.as_str()
        }
    }
}

// Step 2.
// 对字符串切片数组的每个元素应用 capitalize_first，返回字符串向量
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter()
         .map(|&word| capitalize_first(word))  // 遍历每个元素并调用首字母大写函数
         .collect()  // 收集为 Vec<String>
}

// Step 3.
// 对字符串切片数组的每个元素应用 capitalize_first，拼接为单个字符串
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter()
         .map(|&word| capitalize_first(word))  // 遍历每个元素并调用首字母大写函数
         .collect()  // 收集为单个 String（迭代器的 collect 可自动拼接）
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
