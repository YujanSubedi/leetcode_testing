pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: usize = 0;
    let mut right = nums.len() - 1;
    let mut middle: usize;
    while left <= right {
        middle = (left + right) >> 1;
        if nums[middle] < target {
            left = middle + 1;
        } else if nums[middle] > target {
            right = middle - 1;
        } else {
            return middle as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let output_val = 4;
        assert_eq!(search(nums, target), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let output_val = -1;
        assert_eq!(search(nums, target), output_val);
    }
}
