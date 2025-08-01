pub fn length_of_longest_substring(s: String) -> i32 {
    let len_s = s.len();
    let s_as_vec: Vec<char> = s.chars().collect();

    let mut max_len: usize = 0;
    let mut char_hash_map: u128 = 0;

    let mut left: usize = 0;
    let mut right: usize = 0;

    while right < len_s {
        let mask = 1u128 << (s_as_vec[right] as u8);
        while mask & char_hash_map != 0 {
            char_hash_map &= !(1u128 << (s_as_vec[left] as u8));
            left += 1;
        }
        char_hash_map |= mask;
        right += 1;
        if right - left > max_len {
            max_len = right - left;
        }
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let s = "abcabcbb".to_string();
        let output_val = 3;
        assert_eq!(length_of_longest_substring(s), output_val);
    }

    #[test]
    fn test_02() {
        let s = "bbbbb".to_string();
        let output_val = 1;
        assert_eq!(length_of_longest_substring(s), output_val);
    }

    #[test]
    fn test_04() {
        let s = "pwwkew".to_string();
        let output_val = 3;
        assert_eq!(length_of_longest_substring(s), output_val);
    }
}
