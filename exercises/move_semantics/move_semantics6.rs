fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 传递引用，不转移所有权

    string_uppercase(data); // 传递所有权，符合函数要求
}

// 接收引用，不获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 接收所有权（保持不变）
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // 修正：将to_uppercase结果赋值给data

    println!("{}", data);
}
