pub fn add_digits(num: i32) -> i32 {
    let mut curr_num = num;
    let mut next_num = 0;
    while curr_num > 9 {
        while curr_num != 0 {
            next_num += curr_num % 10;
            curr_num /= 10;
        }
        curr_num = next_num;
        next_num = 0;
    }
    curr_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let num = 38;
        let output_val = 2;
        assert_eq!(add_digits(num), output_val);
    }

    #[test]
    fn test_02() {
        let num = 0;
        let output_val = 0;
        assert_eq!(add_digits(num), output_val);
    }
}
