// 定义 Message 枚举，包含所有需要的变体及关联数据
enum Message {
    Quit,                          // 无关联数据：触发退出
    Echo(String),                  // 关联字符串：触发回显
    Move(Point),                   // 关联 Point 结构体：触发移动位置
    ChangeColor(u8, u8, u8),       // 关联 RGB 元组：触发改颜色
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // 核心：用 match 匹配不同的 Message 变体，处理状态更新
        match message {
            Message::Quit => self.quit(), // 匹配 Quit，调用 quit 方法
            Message::Echo(s) => self.echo(s), // 匹配 Echo，提取字符串并调用 echo
            Message::Move(p) => self.move_position(p), // 匹配 Move，提取 Point 并调用 move_position
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)), // 匹配 ChangeColor，提取 RGB 并调用 change_color
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
