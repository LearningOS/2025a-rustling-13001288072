fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // 核心调整：用完y后再声明z，避免可变引用重叠
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
