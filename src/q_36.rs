pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut bitboard_hor: [u16; 9] = [0; 9];
    let mut bitboard_ver: [u16; 9] = [0; 9];
    let mut bitboard_box: [u16; 9] = [0; 9];

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] != '.' {
                let mask = 1u16 << ((board[i][j] as u8) & 15);
                let k = i - i % 3 + j / 3;
                if mask & bitboard_hor[i] != 0
                    || mask & bitboard_ver[j] != 0
                    || mask & bitboard_box[k] != 0
                {
                    return false;
                }
                bitboard_hor[i] |= mask;
                bitboard_ver[j] |= mask;
                bitboard_box[k] |= mask;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let output_val = true;
        assert_eq!(is_valid_sudoku(board), output_val);
    }

    #[test]
    fn test_02() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let output_val = false;
        assert_eq!(is_valid_sudoku(board), output_val);
    }
}
