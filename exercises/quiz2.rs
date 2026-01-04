pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 补全函数签名：输入是Vec<(String, Command)>，输出是Vec<String>
    pub fn transformer(input: &Vec<(String, Command)>) -> Vec<String> {
        // 初始化输出向量，类型为Vec<String>
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // 根据不同命令处理字符串
            let result = match command {
                Command::Uppercase => string.to_uppercase(), // 转大写
                Command::Trim => string.trim().to_string(),   // 去除两端空白
                Command::Append(n) => {                       // 追加n次"bar"
                    let mut s = string.clone();
                    for _ in 0..*n {
                        s.push_str("bar");
                    }
                    s
                }
            };
            output.push(result);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 导入my_module中的transformer函数
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(&vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
