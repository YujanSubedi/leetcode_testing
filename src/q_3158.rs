pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut hash_set: u64 = 0;
    let mut total_xor: i32 = 0;
    for x in nums {
        let mask = 1u64 << (x as u64);
        if hash_set & mask == 0 {
            hash_set |= mask;
        } else {
            total_xor ^= x;
        }
    }
    total_xor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![1, 2, 1, 3];
        let output_val = 1;
        assert_eq!(duplicate_numbers_xor(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![1, 2, 3];
        let output_val = 0;
        assert_eq!(duplicate_numbers_xor(nums), output_val);
    }

    #[test]
    fn test_03() {
        let nums = vec![1, 2, 2, 1];
        let output_val = 3;
        assert_eq!(duplicate_numbers_xor(nums), output_val);
    }
}
