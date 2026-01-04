mod macros {
    // 导出宏，使其能在模块外部被调用
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

// 引入模块内的宏
use macros::my_macro;

fn main() {
    my_macro!();
}
