pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let length: usize = amount as usize + 1;

    let mut solution_table: Vec<i32> = vec![amount + 1; length];
    solution_table[0] = 0;

    for coin in coins {
        let i = coin as usize;
        for j in i..length {
            solution_table[j] = solution_table[j].min(solution_table[j - i] + 1);
        }
    }

    if solution_table[length - 1] > amount {
        -1
    } else {
        solution_table[length - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        let output_val = 3;

        assert_eq!(coin_change(coins, amount), output_val);
    }

    #[test]
    fn test_02() {
        let coins = vec![2];
        let amount = 3;
        let output_val = -1;

        assert_eq!(coin_change(coins, amount), output_val);
    }

    #[test]
    fn test_03() {
        let coins = vec![1];
        let amount = 0;
        let output_val = 0;

        assert_eq!(coin_change(coins, amount), output_val);
    }

    #[test]
    fn test_04() {
        let coins = vec![1, 2, 5];
        let amount = 100;
        let output_val = 20;

        assert_eq!(coin_change(coins, amount), output_val);
    }
}
