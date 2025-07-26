pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let length: usize = amount as usize + 1;

    let mut solution_table: Vec<i32> = vec![0; length];
    solution_table[0] = 1;

    for coin in coins {
        let i = coin as usize;
        for j in i..length {
            solution_table[j] += solution_table[j - i];
        }
    }

    return solution_table[length - 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let amount = 5;
        let coins = vec![1, 2, 5];
        let output_val = 4;

        assert_eq!(change(amount, coins), output_val);
    }

    #[test]
    fn test_02() {
        let amount = 2;
        let coins = vec![3];
        let output_val = 0;

        assert_eq!(change(amount, coins), output_val);
    }

    #[test]
    fn test_03() {
        let amount = 10;
        let coins = vec![10];
        let output_val = 1;

        assert_eq!(change(amount, coins), output_val);
    }
}
