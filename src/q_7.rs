pub fn reverse(x: i32) -> i32 {
    let is_neg = x < 0;
    let mut curr_val: i64 = x as i64;
    curr_val = curr_val.abs();
    let mut res: i64 = 0;

    while curr_val != 0 {
        res = res * 10 + curr_val % 10;
        curr_val /= 10;
    }

    if is_neg {
        res = -res;
    }

    if res > i32::MAX as i64 || res < i32::MIN as i64 {
        0
    } else {
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let x = 123;
        let output_val = 321;
        assert_eq!(reverse(x), output_val);
    }

    #[test]
    fn test_02() {
        let x = -123;
        let output_val = -321;
        assert_eq!(reverse(x), output_val);
    }

    #[test]
    fn test_03() {
        let x = 120;
        let output_val = 21;
        assert_eq!(reverse(x), output_val);
    }
}
