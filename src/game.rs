use crate::board::{Board, Winner};

pub struct Game {
    pub board: Board,
    pub win: Winner,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            win: Winner::None,
        }
    }

    pub fn init_game(&self) {
        println!("Вас приветствует игра Крестики и Нолики!");
        self.board.draw_board();
    }

    pub fn start_game(&mut self) {
        println!("Начинаем играть!");

        loop {
            println!("Ход игрока! Введите число от 1 до 9");

            self.board.step_playrs();
            let result = self.board.check_board();
            if result.0 {
                self.win = result.1;
                return;
            }

            self.board.step_computer();
            let result = self.board.check_board();
            if result.0 {
                self.win = result.1;
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
