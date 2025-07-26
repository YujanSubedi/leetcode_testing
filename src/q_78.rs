pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len_nums = nums.len();

    let mut res: Vec<Vec<i32>> = Vec::with_capacity(1usize << len_nums);
    res.push(Vec::new());

    let mut new_l: usize = 1;
    for val in nums {
        for j in 0..new_l {
            res.push(res[j].clone());
            res[j].push(val);
        }
        new_l <<= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let nums = vec![1, 2, 3];
        let output_val = vec![
            vec![1, 2, 3],
            vec![2, 3],
            vec![1, 3],
            vec![3],
            vec![1, 2],
            vec![2],
            vec![1],
            vec![],
        ];

        assert_eq!(subsets(nums), output_val);
    }

    #[test]
    fn test_02() {
        let nums = vec![0];
        let output_val = vec![vec![0], vec![]];

        assert_eq!(subsets(nums), output_val);
    }
}
