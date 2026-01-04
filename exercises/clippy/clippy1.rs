// clippy1.rs
// 修复 Clippy 警告后的代码
use std::f32::consts::PI; // 引入标准库的 PI 常量

fn main() {
    // 直接使用标准库的 PI，而非手动定义 3.14
    let radius = 5.00f32;

    // 使用更直观的乘法计算平方，替代 f32::powi
    let area = PI * radius * radius;

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
