use rand::Rng;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Winner {
    Player,
    Computer,
    Draw,
    None,
}

// Сущность игрового поля
struct Board {
    char_array: [[char; 3]; 3],
    integer_array: [[u8; 3]; 3],
}

// Методы игрового поля
impl Board {
    fn new() -> Self {
        Board {
            char_array: [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']],
            integer_array: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        }
    }

    pub fn draw_board(&self) {
        for row in self.char_array.iter() {
            for character in row.iter() {
                print!("{}", character);
            }
            println!();
        }
    }

    pub fn check_board(&self) -> (bool, Winner) {
        if let Some(winner) = check_winner(&self.char_array) {
            if winner == 'X' {
                return (true, Winner::Player);
            } else if winner == 'O' {
                return (true, Winner::Computer);
            }
        }

        let filled = self
            .char_array
            .iter()
            .flatten()
            .all(|&c| c == 'X' || c == 'O');

        if filled {
            return (true, Winner::Draw);
        }

        (false, Winner::None)
    }

    pub fn step_playrs(&mut self) {
        let char_X: char = 'X';
        let char_O: char = 'O';

        'step_p: loop {
            let mut number = write();

            for (i, row) in self.integer_array.iter().enumerate() {
                for (j, &val) in row.iter().enumerate() {
                    if val == number {
                        if self.char_array[i][j] != char_X && self.char_array[i][j] != char_O {
                            self.char_array[i][j] = char_X;
                            break 'step_p;
                        }
                    }
                }
            }
            self.draw_board();
            println!("Ошибка ввода! Введите свободное число от 1 до 9");
        }
    }

    pub fn step_computer(&mut self) {
        let char_X: char = 'X';
        let char_O: char = 'O';

        'step: loop {
            let mut rng = rand::thread_rng();
            let n: u8 = rng.gen_range(1..=9);

            for (i, row) in self.integer_array.iter().enumerate() {
                for (j, &val) in row.iter().enumerate() {
                    if val == n {
                        if self.char_array[i][j] != char_X && self.char_array[i][j] != char_O {
                            self.char_array[i][j] = char_O;
                            break 'step;
                        }
                    }
                }
            }
        }

        self.draw_board();
    }
}

// Сущность игры
struct Game {
    board: Board,
    win: Winner,
}

// Методы игры
impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            win: Winner::None,
        }
    }

    pub fn init_game(&self) {
        println!("Вас привествует игра Крестики и Нолики!");
        self.board.draw_board();
    }

    pub fn start_game(&mut self) {
        println!("Начинаем играть!");

        'game: loop {
            println!("Ход игрока! Введите число от 1 до 9");

            self.board.step_playrs();
            let result = self.board.check_board();
            if result.0 {
                self.win = Winner::Player;
                return;
            }

            self.board.step_computer();
            let result = self.board.check_board();
            if result.0 {
                self.win = Winner::Computer;
                return;
            }
        }
    }

    pub fn finish_game(&self) {
        match self.win {
            Winner::Player => println!("Победитель: пользователь!"),
            Winner::Computer => println!("Победитель: компьютер!"),
            Winner::Draw => println!("Ничья!"),
            Winner::None => println!("Игра завершена без результата."),
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.init_game();
    game.start_game();
    game.finish_game();

    close_game();
}

fn close_game() {
    println!("Нажмите Enter чтобы выйти...");
    let mut exit = String::new();
    let _ = io::stdin().read_line(&mut exit);
}

fn write() -> u8 {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Не удалось прочитать строку");
    let number: u8 = number
        .trim()
        .parse()
        .expect("Ошибка преобразования в число");
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
