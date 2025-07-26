pub fn jump(nums: Vec<i32>) -> i32 {
    let final_indx: usize = nums.len();
    let mut left: usize = 0;
    let mut right: usize = 1;
    let mut furthest_jump: usize = 0;

    let mut min_steps: i32 = 0;

    while right < final_indx {
        for i in left..right {
            furthest_jump = furthest_jump.max(i + nums[i] as usize);
        }
        left = right;
        right = furthest_jump + 1;
        min_steps += 1;
    }

    min_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![2, 3, 1, 1, 4];
        let output_val = 2;
        assert_eq!(jump(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![2, 3, 0, 1, 4];
        let output_val = 2;
        assert_eq!(jump(nums), output_val);
    }
}
