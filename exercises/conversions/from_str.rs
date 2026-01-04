use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 空输入字符串
    Empty,
    // 字段数量错误
    BadLen,
    // 姓名字段为空
    NoName,
    // 包装 parse::<usize>() 的错误
    ParseInt(ParseIntError),
}

// 为了让 ? 运算符能自动转换 ParseIntError 为 ParsePersonError
impl From<ParseIntError> for ParsePersonError {
    fn from(err: ParseIntError) -> Self {
        ParsePersonError::ParseInt(err)
    }
}

impl FromStr for Person {
    type Err = ParsePersonError;
    
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 步骤1：空字符串返回 Empty 错误
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        // 步骤2：按逗号分割字符串，收集为 Vec<&str>
        let parts: Vec<&str> = s.split(',').collect();

        // 步骤3：检查分割后的元素数量是否为 2，否则返回 BadLen
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        // 提取姓名和年龄部分，去除首尾空白（更健壮，不影响测试）
        let name = parts[0].trim();
        let age_str = parts[1].trim();

        // 步骤4：姓名为空返回 NoName 错误
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        // 步骤5：解析年龄为 usize，失败则自动转换为 ParseInt 错误（通过 From 实现）
        let age = age_str.parse::<usize>()?;

        // 所有条件满足，返回 Ok(Person)
        Ok(Person {
            name: String::from(name),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p); // Person { name: "Mark", age: 20 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
