#[derive(Debug)]
enum Message {
    // 定义题目中用到的四个枚举变体
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);          // 输出 Quit
    println!("{:?}", Message::Echo);          // 输出 Echo
    println!("{:?}", Message::Move);          // 输出 Move
    println!("{:?}", Message::ChangeColor);   // 输出 ChangeColor
}
