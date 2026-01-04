// 经典C风格结构体：定义具名字段（红、绿、蓝通道）
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// 元组结构体：定义无名字段（按顺序对应红、绿、蓝通道）
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // 实例化经典C风格结构体（指定字段名赋值）
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // 实例化元组结构体（按位置赋值）
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // 实例化单元结构体（无字段，直接写结构体名）
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
