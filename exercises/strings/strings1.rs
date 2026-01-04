fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // 将字符串字面量（&str）转换为String类型，匹配返回值要求
    "blue".to_string()
}
