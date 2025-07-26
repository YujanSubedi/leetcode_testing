pub fn get_smallest_string(n: i32, k: i32) -> String {
    let no_of_z = (k - n) / 25;
    let no_of_a = n - no_of_z - 1;
    let other_char = (k - no_of_z * 26 - no_of_a + 96) as u8;

    println!("{no_of_a}, {other_char}, {no_of_z}");

    if no_of_a >= 0 {
        let mut res_vec = vec![b'a'; no_of_a as usize];
        res_vec.push(other_char);
        let z_vec = vec![b'z'; no_of_z as usize];
        res_vec.extend(&z_vec);
        String::from_utf8(res_vec).unwrap()
    } else {
        let res_vec = vec![b'z'; no_of_z as usize];
        String::from_utf8(res_vec).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let n = 3;
        let k = 27;
        let output_val = "aay".to_string();

        assert_eq!(get_smallest_string(n, k), output_val);
    }

    #[test]
    fn test_02() {
        let n = 5;
        let k = 73;
        let output_val = "aaszz".to_string();

        assert_eq!(get_smallest_string(n, k), output_val);
    }

    #[test]
    fn test_03() {
        let n = 5;
        let k = 130;
        let output_val = "zzzzz".to_string();

        assert_eq!(get_smallest_string(n, k), output_val);
    }

    #[test]
    fn test_04() {
        let n = 24;
        let k = 552;
        let output_val = "aadzzzzzzzzzzzzzzzzzzzzz".to_string();

        assert_eq!(get_smallest_string(n, k), output_val);
    }
}
