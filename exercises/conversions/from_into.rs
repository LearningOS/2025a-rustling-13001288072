#[derive(Debug, PartialEq)] // 补充 PartialEq 方便测试断言（可选，不影响核心逻辑）
struct Person {
    name: String,
    age: usize,
}

// 实现 Default trait（题目已提供，无需修改）
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 核心实现：From<&str> trait
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 步骤1：如果字符串为空，返回默认值
        if s.is_empty() {
            return Person::default();
        }

        // 步骤2：按逗号分割字符串，收集为 Vec<&str>
        let parts: Vec<&str> = s.split(',').collect();

        // 步骤3：检查分割后的部分是否恰好有2个（姓名+年龄），否则返回默认值
        if parts.len() != 2 {
            return Person::default();
        }

        // 提取姓名和年龄部分，去除首尾空白（可选，题目未要求但更健壮）
        let name = parts[0].trim();
        let age_str = parts[1].trim();

        // 步骤4：姓名为空则返回默认值
        if name.is_empty() {
            return Person::default();
        }

        // 步骤5：解析年龄为 usize，失败则返回默认值
        let age = match age_str.parse::<usize>() {
            Ok(num) => num,
            Err(_) => return Person::default(),
        };

        // 所有条件满足，返回实例化的 Person
        Person {
            name: String::from(name),
            age,
        }
    }
}

fn main() {
    // 测试示例
    let p1 = Person::from("Mark,20");
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1); // Person { name: "Mark", age: 20 }
    println!("{:?}", p2); // Person { name: "Gerald", age: 70 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
