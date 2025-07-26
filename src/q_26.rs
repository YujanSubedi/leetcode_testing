pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut res_vec: Vec<i32> = Vec::with_capacity(nums.len());
    res_vec.push(nums[0]);
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            res_vec.push(nums[i]);
        }
    }
    std::mem::swap(nums, &mut res_vec);
    println!("{nums:?}");
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let mut nums = vec![1, 1, 2];
        let output_k = 2;
        assert_eq!(remove_duplicates(&mut nums), output_k);
    }

    #[test]
    fn test_02() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let output_k = 5;
        assert_eq!(remove_duplicates(&mut nums), output_k);
    }
}
