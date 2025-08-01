fn is_symmetric(num: i32) -> bool {
    let mut sum = 0;
    let mut temp_val = num;
    for _ in 0..2 {
        sum += temp_val % 10;
        temp_val /= 10;
    }
    for _ in 0..2 {
        sum -= temp_val % 10;
        temp_val /= 10;
    }
    sum == 0
}

pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    let till_100 = {
        let low_val = low.max(11);
        let temp_val = high.min(99);
        let high_val = temp_val - temp_val % 11;

        if low_val <= high_val {
            (high_val - low_val) / 11 + 1
        } else {
            0
        }
    };

    let till_10000 = {
        let mut res = 0;
        let high_val = high.min(9999);
        let mut low_val = low.max(1001);
        if low_val % 101 != 0 {
            low_val = low_val + 101;
            low_val -= low_val % 101;
        }

        while low_val <= high_val + 101 {
            let next_low = low_val + 101;
            let mut temp_val = low_val;
            while is_symmetric(temp_val) {
                if temp_val <= high_val {
                    res += 1;
                }
                temp_val -= 9;
            }
            let mut temp_val = low_val + 9;
            while is_symmetric(temp_val) && temp_val <= high_val {
                res += 1;
                temp_val += 9;
            }
            low_val = next_low;
        }

        res
    };

    till_100 + till_10000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let low = 1;
        let high = 100;
        let output_val = 9;

        assert_eq!(count_symmetric_integers(low, high), output_val);
    }

    #[test]
    fn test_02() {
        let low = 1200;
        let high = 1230;
        let output_val = 4;

        assert_eq!(count_symmetric_integers(low, high), output_val);
    }

    #[test]
    fn test_03() {
        let low = 100;
        let high = 6034;
        let output_val = 359;

        assert_eq!(count_symmetric_integers(low, high), output_val);
    }

    #[test]
    fn test_04() {
        let low = 100;
        let high = 10000;
        let output_val = 615;

        assert_eq!(count_symmetric_integers(low, high), output_val);
    }

    #[test]
    fn test_05() {
        let low = 12;
        let high = 22;
        let output_val = 1;

        assert_eq!(count_symmetric_integers(low, high), output_val);
    }
}
