pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 2 {
        return nums;
    }

    let unique_bit = {
        let mut total_xor: i32 = 0;
        for x in &nums {
            total_xor ^= x;
        }
        total_xor & (-total_xor)
    };

    let mut first_no: i32 = 0;
    let mut second_no: i32 = 0;

    for x in nums {
        if unique_bit & x == 0 {
            first_no ^= x;
        } else {
            second_no ^= x;
        }
    }
    vec![first_no, second_no]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let output_val = vec![5, 3];
        assert_eq!(single_number(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![-1, 0];
        let output_val = vec![-1, 0];
        assert_eq!(single_number(nums), output_val);
    }

    #[test]
    fn test_03() {
        let nums = vec![0, 1];
        let output_val = vec![0, 1];
        assert_eq!(single_number(nums), output_val);
    }
}
