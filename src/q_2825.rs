pub fn can_make_subsequence(str1: String, str2: String) -> bool {
    let len1 = str1.len();
    let len2 = str2.len();
    if len2 > len1 {
        return false;
    }

    let vec_str1: Vec<char> = str1.chars().collect();
    let vec_str2: Vec<char> = str2.chars().collect();

    let mut index1: usize = 0;
    let mut index2: usize = 0;

    let mut char1: u8 = 0;
    let mut char2: u8 = 0;
    while index1 < len1 && index2 < len2 {
        char2 = vec_str2[index2] as u8;
        while index1 < len1 {
            char1 = vec_str1[index1] as u8;
            index1 += 1;
            if char1 == char2 || (char1 + 1) % 26 == char2 % 26 {
                break;
            }
        }
        index2 += 1;
    }
    if index2 < len2 {
        return false;
    }
    if index1 == len1 && char1 != char2 && (char1 + 1) % 26 != char2 % 26 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let str1 = "abc".to_string();
        let str2 = "ad".to_string();
        assert_eq!(can_make_subsequence(str1, str2), true);
    }

    #[test]
    fn test_02() {
        let str1 = "zc".to_string();
        let str2 = "ad".to_string();
        assert_eq!(can_make_subsequence(str1, str2), true);
    }

    #[test]
    fn test_03() {
        let str1 = "ab".to_string();
        let str2 = "d".to_string();
        assert_eq!(can_make_subsequence(str1, str2), false);
    }

    #[test]
    fn test_04() {
        let str1 = "oh".to_string();
        let str2 = "hu".to_string();
        assert_eq!(can_make_subsequence(str1, str2), false);
    }
}
