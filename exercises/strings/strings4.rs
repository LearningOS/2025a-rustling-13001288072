fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");                  // &str 字面量 → string_slice
    string("red".to_string());             // to_string() 返回 String → string
    string(String::from("hi"));            // String::from 返回 String → string
    string("rust is fun!".to_owned());     // to_owned() 为 &str 创建所有权 → String → string
    string("nice weather".into());         // into() 对 &str 等价于 to_string() → String → string
    string(format!("Interpolation {}", "Station")); // format! 返回 String → string
    string_slice(&String::from("abc")[0..1]); // 切片操作返回 &str → string_slice
    string_slice("  hello there ".trim()); // trim() 返回 &str → string_slice
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace() 返回 String → string
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // to_lowercase() 返回 String → string
}
