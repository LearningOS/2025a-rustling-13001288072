struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), // 添加ref，仅借用不获取所有权
        _ => panic!("no match!"),
    }
    y; // 此时y仍拥有所有权，可正常使用
}
