pub fn add_binary(a: String, b: String) -> String {
    let len_a = a.len();
    let len_b = b.len();
    let max_len = len_a.max(len_b) + 1;

    let a_vec: Vec<char> = a.chars().rev().collect();
    let b_vec: Vec<char> = b.chars().rev().collect();

    let mut result_str: Vec<char> = Vec::with_capacity(max_len);

    let mut a_val: u8;
    let mut b_val: u8;
    let mut carry_val: u8 = 0;
    let mut sum_val: u8;

    for i in 0..max_len {
        if i < len_a {
            a_val = (a_vec[i] as u8) & 1u8;
        } else {
            a_val = 0;
        }
        if i < len_b {
            b_val = (b_vec[i] as u8) & 1u8;
        } else {
            b_val = 0;
        }
        sum_val = a_val ^ b_val ^ carry_val;
        carry_val = (a_val & b_val) | (a_val & carry_val) | (b_val & carry_val);
        result_str.push((sum_val + 48) as char);
    }

    if result_str[max_len - 1] == '0' {
        result_str.pop();
    }

    result_str.reverse();
    result_str.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let a = "11".to_string();
        let b = "1".to_string();
        let output_val = "100".to_string();
        assert_eq!(add_binary(a, b), output_val);
    }

    #[test]
    fn test_02() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let output_val = "10101".to_string();
        assert_eq!(add_binary(a, b), output_val);
    }

    #[test]
    fn test_03() {
        let a = "0".to_string();
        let b = "0".to_string();
        let output_val = "0".to_string();
        assert_eq!(add_binary(a, b), output_val);
    }
}
