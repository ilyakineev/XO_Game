use std::io;

pub fn write() -> u8 {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Не удалось прочитать строку");
    let number: u8 = number.trim().parse().expect("Ошибка преобразования в число");
    number
}

pub fn check_winner(board: &[[char; 3]; 3]) -> Option<char> {
    for row in board.iter() {
        if row[0] == row[1] && row[1] == row[2] {
            return Some(row[0]);
        }
    }

    for col in 0..3 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] {
            return Some(board[0][col]);
        }
    }

    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return Some(board[0][0]);
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return Some(board[0][2]);
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_row_win() {
        let board = [
            ['X', 'X', 'X'],
            ['4', '5', '6'],
            ['7', '8', '9']
        ];
        assert_eq!(check_winner(&board), Some('X'));
    }

    #[test]
    fn test_column_win() {
        let board = [
            ['O', '2', '3'],
            ['O', '5', '6'],
            ['O', '8', '9']
        ];
        assert_eq!(check_winner(&board), Some('O'));
    }

    #[test]
    fn test_diagonal_win() {
        let board = [
            ['X', '2', 'O'],
            ['4', 'X', '6'],
            ['O', '8', 'X']
        ];
        assert_eq!(check_winner(&board), Some('X'));
    }

    #[test]
    fn test_anti_diagonal_win() {
        let board = [
            ['1', '2', 'O'],
            ['4', 'O', '6'],
            ['O', '8', '9']
        ];
        assert_eq!(check_winner(&board), Some('O'));
    }

    #[test]
    fn test_no_winner() {
        let board = [
            ['1', '2', '3'],
            ['4', '5', '6'],
            ['7', '8', '9']
        ];
        assert_eq!(check_winner(&board), None);
    }

    #[test]
    fn test_draw() {
        let board = [
            ['X', 'O', 'X'],
            ['X', 'O', 'O'],
            ['O', 'X', 'X']
        ];
        assert_eq!(check_winner(&board), None);
    }
}
