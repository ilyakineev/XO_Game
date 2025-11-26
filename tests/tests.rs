use xo_game::check_winner;


#[cfg(test)]
mod tests {

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