fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // 解构元组，将两个元素分别绑定到name和age变量

    println!("{} is {} years old.", name, age);
}
