pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut state_1 = 0;
    let mut state_2 = 0;
    for x in nums {
        (state_1, state_2) = (state_2 & x | state_1 & !x, !state_1 & (state_2 ^ x));
    }
    state_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![2, 2, 3, 2];
        let output_val = 3;
        assert_eq!(single_number(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        let output_val = 99;
        assert_eq!(single_number(nums), output_val);
    }
}
