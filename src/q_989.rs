pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
    let len_nums = num.len();
    let mut res_vec: Vec<i32> = Vec::with_capacity(len_nums + 1);

    let mut index: usize = len_nums;
    let mut sum_val = k;
    while sum_val != 0 || index > 0 {
        if index > 0 {
            index -= 1;
            sum_val += num[index];
        }
        res_vec.push(sum_val % 10);
        sum_val /= 10;
    }

    res_vec.reverse();
    res_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let num = vec![1, 2, 0, 0];
        let k = 34;
        let output_val = vec![1, 2, 3, 4];

        assert_eq!(add_to_array_form(num, k), output_val);
    }

    #[test]
    fn test_02() {
        let num = vec![2, 7, 4];
        let k = 181;
        let output_val = vec![4, 5, 5];

        assert_eq!(add_to_array_form(num, k), output_val);
    }

    #[test]
    fn test_03() {
        let num = vec![2, 1, 5];
        let k = 806;
        let output_val = vec![1, 0, 2, 1];

        assert_eq!(add_to_array_form(num, k), output_val);
    }

    #[test]
    fn test_04() {
        let num = vec![9, 9, 9, 9, 9, 9, 9];
        let k = 1;
        let output_val = vec![1, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(add_to_array_form(num, k), output_val);
    }

    #[test]
    fn test_05() {
        let num = vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
        let k = 1;
        let output_val = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(add_to_array_form(num, k), output_val);
    }
}
