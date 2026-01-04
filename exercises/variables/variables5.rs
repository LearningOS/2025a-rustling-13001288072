fn main() {
    // 1. 声明变量number，绑定字符串值"T-H-R-E-E"（类型：&str）
    let number = "T-H-R-E-E"; // don't change this line
    // 2. 打印字符串：输出 "Spell a Number : T-H-R-E-E"
    println!("Spell a Number : {}", number);
    
    // 3. 变量遮蔽：用let重新声明同名变量number，绑定整数值3（类型：i32）
    let number = 3; // don't rename this variable
    // 4. 打印计算结果：3 + 2 = 5，输出 "Number plus two is : 5"
    println!("Number plus two is : {}", number + 2);
}
