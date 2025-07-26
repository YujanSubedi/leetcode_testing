pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
    if p == 0 {
        return 0;
    }

    let sorted_nums = {
        let mut temp_vec = nums.clone();
        temp_vec.sort();
        temp_vec
    };

    let len_n = sorted_nums.len();
    let mut left = 0;
    let mut right = sorted_nums[len_n - 1] - sorted_nums[0];

    while left < right {
        let middle = (left + right) >> 1;
        let mut rem_pair = p;
        let mut indx = 1;
        while indx < len_n {
            if middle >= sorted_nums[indx] - sorted_nums[indx - 1] {
                rem_pair -= 1;
                indx += 1;
            }
            indx += 1;
        }
        if rem_pair < 1 {
            right = middle;
        } else {
            left = middle + 1;
        }
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![10, 1, 2, 7, 1, 3];
        let p = 2;
        let output_val = 1;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![4, 2, 1, 2];
        let p = 1;
        let output_val = 0;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_03() {
        let nums = vec![4, 0, 2, 1, 2, 5, 5, 3];
        let p = 3;
        let output_val = 1;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_04() {
        let nums = vec![3, 4, 2, 3, 2, 1, 2];
        let p = 3;
        let output_val = 1;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_05() {
        let nums = vec![1, 2];
        let p = 1;
        let output_val = 1;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_06() {
        let nums = vec![3, 6, 11, 6, 8, 6, 5, 3, 3, 7];
        let p = 3;
        let output_val = 1;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_07() {
        let nums = vec![8, 9, 1, 5, 4, 3, 6, 4, 3, 7];
        let p = 4;
        let output_val = 1;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_08() {
        let nums = vec![3, 3, 5, 1, 0, 5, 6, 6];
        let p = 4;
        let output_val = 1;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_09() {
        let nums = vec![7, 7, 16, 7, 7, 2, 6, 5, 5, 0];
        let p = 3;
        let output_val = 0;

        assert_eq!(minimize_max(nums, p), output_val);
    }

    #[test]
    fn test_12() {
        let nums = vec![
            17409, 105373, 27399, 82602, 109086, 24648, 30503, 20572, 67038, 86324, 12410, 74618,
            51020, 59228, 15342, 81354, 38849, 16780, 35500, 91344, 39666, 76870, 74590, 105190,
            109106, 4466, 10867, 7345, 50872, 43825, 32880, 86755, 13907, 65384, 92043, 109481,
            13573, 65888, 70704, 11606, 2918, 94611, 45908, 74209, 56260, 9744, 7263, 40556,
        ];

        let p = 10;
        let output_val = 699;

        assert_eq!(minimize_max(nums, p), output_val);
    }
}
