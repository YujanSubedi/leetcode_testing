pub fn count_bits(n: i32) -> Vec<i32> {
    if n == 0 {
        return vec![0];
    } else if n == 1 {
        return vec![0, 1];
    }

    let mut res_vec: Vec<i32> = Vec::with_capacity(n as usize + 1);
    res_vec.push(0);
    res_vec.push(1);

    for i in 2..=n as usize {
        if i & 1 == 0 {
            res_vec.push(res_vec[i >> 1]);
        } else {
            res_vec.push(res_vec[i >> 1] + 1);
        }
    }

    res_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let n = 2;
        let output_vec = vec![0, 1, 1];
        assert_eq!(count_bits(n), output_vec);
    }

    #[test]
    fn test_02() {
        let n = 5;
        let output_vec = vec![0, 1, 1, 2, 1, 2];
        assert_eq!(count_bits(n), output_vec);
    }
}
