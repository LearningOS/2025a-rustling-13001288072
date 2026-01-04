// build.rs
fn main() {
    // 1. 兼容tests7：设置TEST_FOO环境变量（原逻辑保留）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 2. 兼容tests8：向Cargo输出指令，启用"pass"特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
