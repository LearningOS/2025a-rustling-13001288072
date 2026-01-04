fn main() {
    let word = String::from("green"); // Try not changing this line :)
    // 将String转换为&str（借用），匹配函数参数要求
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
