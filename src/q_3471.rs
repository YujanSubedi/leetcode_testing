pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    let mut hash_map = {
        let len_nums = nums.len();
        if len_nums == 1 {
            return nums[0];
        }
        if k == 1 {
            let mut hash_map: u64 = 0;
            let mut only_once_hash_map: u64 = 0;
            for val in nums {
                let mask = 1u64 << (val as u64);
                if hash_map & mask == 0 {
                    only_once_hash_map |= mask;
                    hash_map |= mask;
                } else {
                    only_once_hash_map &= !mask;
                }
            }
            if only_once_hash_map == 0 {
                return -1;
            }
            only_once_hash_map
        } else if k as usize == len_nums {
            let mut hash_map: u64 = 0;
            for val in nums {
                let mask = 1u64 << (val as u64);
                hash_map |= mask;
            }
            hash_map
        } else {
            if nums[0] == nums[len_nums - 1] {
                return -1;
            }
            let mut hash_map: u64 = 0;
            hash_map |= 1u64 << (nums[0] as u64);
            hash_map |= 1u64 << (nums[len_nums - 1] as u64);

            for i in 1..len_nums - 1 {
                let mask = 1u64 << (nums[i] as u64);
                if hash_map & mask != 0 {
                    hash_map &= !mask;
                    if hash_map == 0 {
                        return -1;
                    }
                }
            }
            hash_map
        }
    };

    let mut res = 0;
    let mut count = 0;
    while hash_map != 0 {
        if hash_map & 1u64 == 1 {
            res = count;
        }
        count += 1;
        hash_map >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![3, 9, 2, 1, 7];
        let k = 3;
        let output_val = 7;
        assert_eq!(largest_integer(nums, k), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![3, 9, 7, 2, 1, 7];
        let k = 4;
        let output_val = 3;
        assert_eq!(largest_integer(nums, k), output_val);
    }

    #[test]
    fn test_03() {
        let nums = vec![0, 0];
        let k = 1;
        let output_val = -1;
        assert_eq!(largest_integer(nums, k), output_val);
    }

    #[test]
    fn test_04() {
        let nums = vec![0, 0];
        let k = 2;
        let output_val = 0;
        assert_eq!(largest_integer(nums, k), output_val);
    }
}
