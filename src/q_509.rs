pub fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        a = a + b;
        b = a - b;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let n = 2;
        let output_val = 1;
        assert_eq!(fib(n), output_val);
    }

    #[test]
    fn test_02() {
        let n = 3;
        let output_val = 2;
        assert_eq!(fib(n), output_val);
    }

    #[test]
    fn test_03() {
        let n = 4;
        let output_val = 3;
        assert_eq!(fib(n), output_val);
    }
}
