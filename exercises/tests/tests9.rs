extern "Rust" {
    // 为my_demo_function添加link_name属性，指向Foo::my_demo_function
    #[link_name = "Foo::my_demo_function"]
    fn my_demo_function(a: u32) -> u32;
    // 为my_demo_function_alias添加link_name属性，同样指向Foo::my_demo_function（别名）
    #[link_name = "Foo::my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // 为函数添加no_mangle属性，禁止符号混淆，确保外部能通过名称找到该函数
    #[no_mangle]
    // No `extern` equals `extern "Rust"`.
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
