#[derive(Debug)]
enum Message {
    // 定义带不同关联数据的枚举变体
    Quit,                          // 无关联数据
    Echo(String),                  // 关联字符串（元组风格）
    Move { x: i32, y: i32 },       // 关联结构体风格的数据
    ChangeColor(u8, u8, u8),       // 关联三个 u8（元组风格，RGB）
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
