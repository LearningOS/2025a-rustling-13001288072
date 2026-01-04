fn main() {
    // 1. 声明不可变变量x，绑定值10（Rust自动推导类型为i32）
    let x = 10;
    
    // 2. 条件判断：检查x是否等于10（Rust的if条件无需括号，且必须是布尔值）
    if x == 10 {
        // 条件成立时执行（本例中必然走这里）
        println!("x is ten!");
    } else {
        // 条件不成立时执行（本例不会触发）
        println!("x is not ten!");
    }
}
