fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 替换冗余的 for 循环，用 if let 匹配 Option 中的值
    if let Some(x) = option {
        res += x;
    }
    
    println!("{}", res);
}
