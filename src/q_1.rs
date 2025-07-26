use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index_hash_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for (i, val) in nums.iter().enumerate() {
        match index_hash_map.get(val) {
            Some(indx) => {
                return vec![*indx as i32, i as i32];
            }
            None => {
                index_hash_map.insert(target - val, i);
            }
        }
    }
    vec![0, 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let output_val = vec![0, 1];
        assert_eq!(two_sum(nums, target), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let output_val = vec![1, 2];
        assert_eq!(two_sum(nums, target), output_val);
    }

    #[test]
    fn test_03() {
        let nums = vec![3, 3];
        let target = 6;
        let output_val = vec![0, 1];
        assert_eq!(two_sum(nums, target), output_val);
    }
}
