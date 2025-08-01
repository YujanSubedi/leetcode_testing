pub fn my_atoi(s: String) -> i32 {
    let len_s = s.len();
    let s_as_vec: Vec<char> = s.chars().collect();

    let mut indx = 0;
    while indx < len_s && s_as_vec[indx] == ' ' {
        indx += 1;
    }
    if indx >= len_s {
        return 0;
    }

    let is_neg: bool;
    if s_as_vec[indx] == '-' {
        is_neg = true;
        indx += 1;
    } else if s_as_vec[indx] == '+' {
        is_neg = false;
        indx += 1;
    } else {
        is_neg = false;
    }

    let i32_max: i64 = i32::MAX as i64;
    let i32_min: i64 = i32::MIN as i64;
    let mut res: i64 = 0;

    while indx < len_s {
        let val = (s_as_vec[indx] as u8) as i64 - 48;
        if val < 0 || val > 9 || res > i32_max {
            break;
        }
        res = res * 10 + val;
        indx += 1;
    }

    if is_neg {
        res = -res;
    }

    if res > i32_max {
        i32::MAX
    } else if res < i32_min {
        i32::MIN
    } else {
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let s = "42".to_string();
        let output_val = 42;
        assert_eq!(my_atoi(s), output_val);
    }

    #[test]
    fn test_02() {
        let s = " -042".to_string();
        let output_val = -42;
        assert_eq!(my_atoi(s), output_val);
    }

    #[test]
    fn test_03() {
        let s = "1337c0d3".to_string();
        let output_val = 1337;
        assert_eq!(my_atoi(s), output_val);
    }

    #[test]
    fn test_04() {
        let s = "0-1".to_string();
        let output_val = 0;
        assert_eq!(my_atoi(s), output_val);
    }

    #[test]
    fn test_05() {
        let s = "words and 987".to_string();
        let output_val = 0;
        assert_eq!(my_atoi(s), output_val);
    }
}
