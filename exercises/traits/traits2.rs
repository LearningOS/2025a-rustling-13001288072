trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为Vec<String>实现AppendBar trait
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        // 复制self到可变变量中（因为self是不可变的）
        let mut vec = self;
        // 向向量末尾添加字符串"Bar"
        vec.push(String::from("Bar"));
        // 返回修改后的向量
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
