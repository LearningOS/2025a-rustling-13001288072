#[allow(unused_variables)] // 移除了 unused_assignments，因为修复后无无效赋值
fn main() {
    let my_option: Option<()> = None;
    // 修复：移除无意义且会 panic 的 unwrap() 调用
    if my_option.is_none() {
        // 原逻辑是判断 None，此时 unwrap 必然 panic，直接移除该调用
        println!("my_option is None, skipping unwrap");
    }

    // 修复：数组元素间补充缺失的逗号
    let my_arr = &[
        -1, -2, -3, // 补充逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修复：正确创建空 Vec（而非错误使用 resize）
    // 方式1：直接创建空 Vec（更简洁）
    let my_empty_vec: Vec<i32> = Vec::new();
    // 方式2（若要保留 resize 逻辑）：先创建 Vec，再调用 resize（不赋值）
    // let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // my_empty_vec.resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 修复：使用 std::mem::swap 正确交换变量（Clippy 推荐的方式）
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
