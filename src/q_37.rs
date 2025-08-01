fn solve_sudoku_helper(
    board: &mut Vec<Vec<char>>,
    bitboard_hor: &mut [u16; 9],
    bitboard_ver: &mut [u16; 9],
    bitboard_box: &mut [u16; 9],
    mut i: usize,
    mut j: usize,
) -> bool {
    while i < 9 {
        if board[i][j] == '.' {
            for val in 1..10u8 {
                let mask = 1u16 << val;
                let k = i - i % 3 + j / 3;
                if mask & bitboard_hor[i] != 0
                    || mask & bitboard_ver[j] != 0
                    || mask & bitboard_box[k] != 0
                {
                    continue;
                }

                board[i][j] = (val + 48) as char;
                bitboard_hor[i] |= mask;
                bitboard_ver[j] |= mask;
                bitboard_box[k] |= mask;
                if solve_sudoku_helper(board, bitboard_hor, bitboard_ver, bitboard_box, i, j) {
                    return true;
                }
                bitboard_hor[i] &= !mask;
                bitboard_ver[j] &= !mask;
                bitboard_box[k] &= !mask;
            }

            board[i][j] = '.';
            return false;
        }

        i += j >> 3;
        j += 1;
        j %= 9;
    }
    true
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut bitboard_hor: [u16; 9] = [0; 9];
    let mut bitboard_ver: [u16; 9] = [0; 9];
    let mut bitboard_box: [u16; 9] = [0; 9];

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] != '.' {
                let mask = 1u16 << ((board[i][j] as u8) & 15);
                let k = i - i % 3 + j / 3;
                bitboard_hor[i] |= mask;
                bitboard_ver[j] |= mask;
                bitboard_box[k] |= mask;
            }
        }
    }

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                solve_sudoku_helper(
                    board,
                    &mut bitboard_hor,
                    &mut bitboard_ver,
                    &mut bitboard_box,
                    i,
                    j,
                );
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let mut board = vec![
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

        let output_val = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        solve_sudoku(&mut board);
        assert_eq!(board, output_val);
    }
}
