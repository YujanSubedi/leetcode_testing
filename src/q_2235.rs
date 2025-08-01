pub fn sum(num1: i32, num2: i32) -> i32 {
    let carr = {
        let mut cur_c = -1;
        let mut next_c = 0;
        while cur_c != next_c {
            cur_c = next_c;
            next_c = ((num1 & num2) | (num2 & cur_c) | (cur_c & num1)) << 1;
        }
        cur_c
    };

    num1 ^ num2 ^ carr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let num1 = 12;
        let num2 = 5;
        let output_val = 17;

        assert_eq!(sum(num1, num2), output_val);
    }

    #[test]
    fn test_02() {
        let num1 = -10;
        let num2 = 4;
        let output_val = -6;

        assert_eq!(sum(num1, num2), output_val);
    }
}
