use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
enum IntoColorError {
    // 切片长度错误
    BadLen,
    // 整数转换错误（值超出 0..=255 范围）
    IntConversion,
}

// 辅助函数：校验 i16 值是否能安全转换为 u8（0..=255）
fn validate_rgb_value(val: i16) -> Result<u8, IntoColorError> {
    if val >= 0 && val <= 255 {
        Ok(val as u8)
    } else {
        Err(IntoColorError::IntConversion)
    }
}

// 1. 元组 (i16, i16, i16) 实现 TryFrom
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let red = validate_rgb_value(tuple.0)?;
        let green = validate_rgb_value(tuple.1)?;
        let blue = validate_rgb_value(tuple.2)?;
        
        Ok(Color { red, green, blue })
    }
}

// 2. 数组 [i16; 3] 实现 TryFrom
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let red = validate_rgb_value(arr[0])?;
        let green = validate_rgb_value(arr[1])?;
        let blue = validate_rgb_value(arr[2])?;
        
        Ok(Color { red, green, blue })
    }
}

// 3. 切片 &[i16] 实现 TryFrom
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        // 第一步：校验切片长度是否为 3
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        
        // 第二步：校验每个值的范围并转换
        let red = validate_rgb_value(slice[0])?;
        let green = validate_rgb_value(slice[1])?;
        let blue = validate_rgb_value(slice[2])?;
        
        Ok(Color { red, green, blue })
    }
}

fn main() {
    // 测试元组转换
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1); // Ok(Color { red: 183, green: 65, blue: 14 })

    // 测试数组转换（TryInto）
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2); // Ok(Color { red: 183, green: 65, blue: 14 })

    // 测试切片转换
    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3); // Ok(Color { red: 183, green: 65, blue: 14 })
    
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4); // Ok(Color { red: 183, green: 65, blue: 14 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
