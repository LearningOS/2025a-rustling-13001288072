fn main() {
    call_me(3);
}

// 函数接收 i32 类型参数 num，循环打印指定次数的呼叫信息
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
