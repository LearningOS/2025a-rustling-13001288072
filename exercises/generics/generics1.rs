fn main() {
    // 替换通配符 ? 为具体的类型 &str（字符串切片），匹配 push 的值类型
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
