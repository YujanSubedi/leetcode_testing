pub fn climb_stairs(n: i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    for _ in 0..n {
        b = a + b;
        a = b - a;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let n = 2;
        let output_val = 2;
        assert_eq!(climb_stairs(n), output_val);
    }

    #[test]
    fn test_02() {
        let n = 3;
        let output_val = 3;
        assert_eq!(climb_stairs(n), output_val);
    }
}
