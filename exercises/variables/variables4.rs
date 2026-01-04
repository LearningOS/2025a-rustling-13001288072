fn main() {
    // 1. 声明可变变量x，绑定初始值3（mut关键字表示变量可修改）
    let mut x = 3;
    // 2. 打印初始值：输出 "Number 3"
    println!("Number {}", x);
    // 3. 修改可变变量x的值为5（只有mut变量才能重新赋值）
    x = 5; // don't change this line
    // 4. 打印修改后的值：输出 "Number 5"
    println!("Number {}", x);
}
