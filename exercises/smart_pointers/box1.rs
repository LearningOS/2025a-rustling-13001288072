#[derive(PartialEq, Debug)]
pub enum List {
    // Step 1: 使用Box包装递归的List类型，将递归部分存储到堆上
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    // Step 2: 创建空的cons列表（Nil变体）
    List::Nil
}

pub fn create_non_empty_list() -> List {
    // Step 2: 创建非空的cons列表（Cons变体，包含值和下一个节点）
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
