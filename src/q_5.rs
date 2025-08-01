pub fn longest_palindrome(s: String) -> String {
    let len_s = s.len();
    let s_as_vec: Vec<char> = s.chars().collect();

    let mut largest_left: usize = 0;
    let mut largest_right: usize = 1;

    let mut max_l: usize = 0;

    for i in 1..len_s {
        let mut ev_l = 1;
        while i >= ev_l && i + ev_l - 1 < len_s && s_as_vec[i - ev_l] == s_as_vec[i + ev_l - 1] {
            ev_l += 1;
        }
        ev_l -= 1;

        if ev_l > max_l {
            max_l = ev_l;
            largest_left = i - ev_l;
            largest_right = i + ev_l;
        }

        let mut od_l = 1;
        while i >= od_l && i + od_l < len_s && s_as_vec[i - od_l] == s_as_vec[i + od_l] {
            od_l += 1;
        }
        od_l -= 1;

        if od_l >= max_l {
            max_l = od_l;
            largest_left = i - od_l;
            largest_right = i + od_l + 1;
        }
    }

    let res = &s[largest_left..largest_right];
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let s = "babad".to_string();
        let output_val = "aba".to_string();
        assert_eq!(longest_palindrome(s), output_val);
    }

    #[test]
    fn test_02() {
        let s = "cbbd".to_string();
        let output_val = "bb".to_string();
        assert_eq!(longest_palindrome(s), output_val);
    }

    #[test]
    fn test_03() {
        let s = "a".to_string();
        let output_val = "a".to_string();
        assert_eq!(longest_palindrome(s), output_val);
    }

    #[test]
    fn test_04() {
        let s = "ccc".to_string();
        let output_val = "ccc".to_string();
        assert_eq!(longest_palindrome(s), output_val);
    }
}
