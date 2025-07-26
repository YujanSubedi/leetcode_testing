pub fn rob(nums: Vec<i32>) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    for num in nums {
        (left, right) = (right, right.max(left + num));
    }
    right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![1, 2, 3, 1];
        let output_val = 4;
        assert_eq!(rob(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![2, 7, 9, 3, 1];
        let output_val = 12;
        assert_eq!(rob(nums), output_val);
    }
}
