pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut curr: i64 = x as i64;
    let mut res: i64 = 0;
    while curr != 0 {
        res = res * 10 + curr % 10;
        curr /= 10;
    }
    if res > i32::MAX as i64 {
        false
    } else {
        x as i64 == res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let x = 121;
        let output_val = true;
        assert_eq!(is_palindrome(x), output_val);
    }

    #[test]
    fn test_02() {
        let x = -121;
        let output_val = false;
        assert_eq!(is_palindrome(x), output_val);
    }

    #[test]
    fn test_03() {
        let x = 10;
        let output_val = false;
        assert_eq!(is_palindrome(x), output_val);
    }
}
