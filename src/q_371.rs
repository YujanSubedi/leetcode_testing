pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut curr_carry = -1;
    let mut next_carry = 0;
    while next_carry != curr_carry {
        curr_carry = next_carry;
        next_carry = ((a & b) | (a & curr_carry) | (b & curr_carry)) << 1;
    }
    a ^ b ^ curr_carry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let a = 1;
        let b = 2;
        let output_val = 3;
        assert_eq!(get_sum(a, b), output_val);
    }

    #[test]
    fn test_02() {
        let a = 2;
        let b = 3;
        let output_val = 5;
        assert_eq!(get_sum(a, b), output_val);
    }
}
