pub fn can_jump(nums: Vec<i32>) -> bool {
    let final_index: usize = nums.len() - 1;
    let mut left: usize = 0;
    let mut right: usize = 0;
    while right >= left {
        let furtherst_jump_index: usize = left + nums[left] as usize;
        if furtherst_jump_index >= final_index {
            return true;
        }
        right = right.max(furtherst_jump_index);
        left += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![2, 3, 1, 1, 4];
        let output_val = true;

        assert_eq!(can_jump(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![3, 2, 1, 0, 4];
        let output_val = false;

        assert_eq!(can_jump(nums), output_val);
    }
}
