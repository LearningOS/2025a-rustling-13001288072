// build.rs
fn main() {
    // 获取当前时间戳（秒级）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 向Cargo输出指令，设置环境变量 TEST_FOO 为当前时间戳
    // 这样测试代码就能读取到这个变量，且满足 timestamp ∈ [e, e+10)
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}
