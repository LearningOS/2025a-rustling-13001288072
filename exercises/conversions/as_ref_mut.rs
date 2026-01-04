// AsRef and AsMut allow for cheap reference-to-reference conversions.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    // AsRef<str> 保证 arg 能转为 &str，再转字节数组求长度
    arg.as_ref().as_bytes().len()
}

fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    // AsRef<str> 保证 arg 能转为 &str，再遍历字符计数
    arg.as_ref().chars().count()
}

// 为 T 添加 AsMut<u32> trait bound，保证能转为 &mut u32
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // 通过 as_mut() 获取 u32 的可变引用，然后平方
    let num_ref = arg.as_mut();
    *num_ref = (*num_ref) * (*num_ref);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
