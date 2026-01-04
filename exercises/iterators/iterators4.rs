pub fn factorial(num: u64) -> u64 {
    // 核心逻辑：用迭代器生成 1..=num 的序列，再折叠（fold）计算乘积
    // 初始值 1（阶乘的基准值），闭包中累积乘积
    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
