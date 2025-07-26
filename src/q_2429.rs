pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let (bits_no1, bits_no2) = {
        let mut count1 = 0;
        let mut num1_val = num1;
        while num1_val != 0 {
            num1_val &= num1_val - 1;
            count1 += 1;
        }

        let mut count2 = 0;
        let mut num2_val = num2;
        while num2_val != 0 {
            num2_val &= num2_val - 1;
            count2 += 1;
        }
        (count1, count2)
    };

    if bits_no1 > bits_no2 {
        let mut res_no = num1;
        for _ in 0..(bits_no1 - bits_no2) {
            res_no &= res_no - 1;
        }
        res_no
    } else if bits_no1 < bits_no2 {
        let mut res_no = num1;
        for _ in 0..(bits_no2 - bits_no1) {
            res_no |= res_no + 1;
        }
        res_no
    } else {
        num1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let num1 = 3;
        let num2 = 5;
        let output_val = 3;

        assert_eq!(minimize_xor(num1, num2), output_val);
    }

    #[test]
    fn test_02() {
        let num1 = 1;
        let num2 = 12;
        let output_val = 3;

        assert_eq!(minimize_xor(num1, num2), output_val);
    }
}
