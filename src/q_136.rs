pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut total_xor: i32 = 0;
    for x in nums {
        total_xor ^= x;
    }
    total_xor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![2, 2, 1];
        let output_val = 1;
        assert_eq!(single_number(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![4, 1, 2, 1, 2];
        let output_val = 4;
        assert_eq!(single_number(nums), output_val);
    }

    #[test]
    fn test_03() {
        let nums = vec![1];
        let output_val = 1;
        assert_eq!(single_number(nums), output_val);
    }
}
