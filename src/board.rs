use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Winner {
    Player,
    Computer,
    Draw,
    None,
}

pub struct Board {
    pub char_array: [[char; 3]; 3],
    pub integer_array: [[u8; 3]; 3],
    pub char_x: char,
    pub char_o: char,
}

impl Board {
    pub fn new() -> Self {
        Board {
            char_array: [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']],
            integer_array: [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            char_x: 'X',
            char_o: 'O',
        }
    }

    pub fn draw_board(&self) {
        for (i, row) in self.char_array.iter().enumerate() {
            println!(" {} | {} | {} ", row[0], row[1], row[2]);
            if i < 2 {
                println!("-----------");
            }
        }
    }

    pub fn check_board(&self) -> (bool, Winner) {
        if let Some(winner) = crate::utils::check_winner(&self.char_array) {
            if winner == 'X' {
                return (true, Winner::Player);
            } else if winner == 'O' {
                return (true, Winner::Computer);
            }
        }

        let filled = self.char_array.iter().flatten().all(|&c| c == 'X' || c == 'O');

        if filled {
            return (true, Winner::Draw);
        }

        (false, Winner::None)
    }

    pub fn step_playrs(&mut self) {
        'step_p: loop {
            let number = crate::utils::write();

            for (i, row) in self.integer_array.iter().enumerate() {
                for (j, &val) in row.iter().enumerate() {
                    if val == number {
                        if self.char_array[i][j] != self.char_x && self.char_array[i][j] != self.char_o {
                            self.char_array[i][j] = self.char_x;
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
        'step: loop {
            let mut rng = rand::thread_rng();
            let n: u8 = rng.gen_range(1..=9);

            for (i, row) in self.integer_array.iter().enumerate() {
                for (j, &val) in row.iter().enumerate() {
                    if val == n {
                        if self.char_array[i][j] != self.char_x && self.char_array[i][j] != self.char_o {
                            self.char_array[i][j] = self.char_o;
                            break 'step;
                        }
                    }
                }
            }
        }

        self.draw_board();
    }
}
