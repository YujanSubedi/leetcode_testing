pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let first_selected = {
        let mut left: i32 = 0;
        let mut right: i32 = 0;
        for val in nums.iter().take(nums.len() - 1) {
            (left, right) = (right, right.max(left + val));
        }
        right
    };

    let first_not_selected = {
        let mut left: i32 = 0;
        let mut right: i32 = 0;
        for val in nums.iter().skip(1) {
            (left, right) = (right, right.max(left + val));
        }
        right
    };

    first_selected.max(first_not_selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![2, 3, 2];
        let output_val = 3;
        assert_eq!(rob(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![1, 2, 3, 1];
        let output_val = 4;
        assert_eq!(rob(nums), output_val);
    }

    #[test]
    fn test_03() {
        let nums = vec![1, 2, 3];
        let output_val = 3;
        assert_eq!(rob(nums), output_val);
    }
}
