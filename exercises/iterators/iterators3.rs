#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// 实现除法逻辑：处理除零、无法整除、正常整除三种情况
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    // 1. 优先处理除零错误
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    // 2. 检查是否能整除
    if a % b != 0 {
        return Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }));
    }
    // 3. 能整除则返回商
    Ok(a / b)
}

// 收集所有除法结果为单个Result（全部成功返回Ok(Vec)，否则返回第一个错误）
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    // map处理每个元素的除法，collect自动合并为Result<Vec, E>
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

// 收集所有除法结果为Vec<Result>（保留每个元素的独立Result状态）
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    // map处理每个元素的除法，collect直接收集为Vec<Result>
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
