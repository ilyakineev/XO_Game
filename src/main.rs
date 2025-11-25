use rand::Rng;
use std::io;

fn main() {
    let mut char_array: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let integer_array: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let char_X = 'X';
    let char_O = 'O';

    println!("Вас привествует игра Крестики и Нолики!");

    println!("Начинаем играть!");

    for row in char_array.iter() {
        for character in row.iter() {
            print!("{}", character);
        }
        println!();
    }

    'game: loop {
        println!("Ход игрока! Введите число от 1 до 9");
        'step_p: loop {
            let mut number = String::new();
            io::stdin()
                .read_line(&mut number)
                .expect("Не удалось прочитать строку");
            let number: u8 = number
                .trim()
                .parse()
                .expect("Ошибка преобразования в число");

            for (i, row) in integer_array.iter().enumerate() {
                for (j, &val) in row.iter().enumerate() {
                    if val == number {
                        if char_array[i][j] != char_X && char_array[i][j] != char_O {
                            char_array[i][j] = char_X;
                            break 'step_p;
                        }
                    }
                }
            }

            for row in char_array.iter() {
                for character in row.iter() {
                    print!("{}", character);
                }
                println!();
            }
            println!("Ошибка ввода! Введите свободное число от 1 до 9");
        }

        if let Some(winner) = check_winner(&char_array) {
            if winner == char_X {
                println!("Победитель игрок!");
            } else {
                println!("Победитель компьютер!");
            }
            break 'game;
        }

        println!("Ход компьютера!");

        'step: loop {
            let mut rng = rand::thread_rng();
            let n: u8 = rng.gen_range(1..=9);

            for (i, row) in integer_array.iter().enumerate() {
                for (j, &val) in row.iter().enumerate() {
                    if val == n {
                        if char_array[i][j] != char_X && char_array[i][j] != char_O {
                            char_array[i][j] = char_O;
                            break 'step;
                        }
                    }
                }
            }
        }

        for row in char_array.iter() {
            for character in row.iter() {
                print!("{}", character);
            }
            println!();
        }

        if let Some(winner) = check_winner(&char_array) {
            if winner == char_X {
                println!("Победитель игрок!");
            } else {
                println!("Победитель компьютер!");
            }
            break 'game;
        }
    }

    println!("Нажмите Enter чтобы выйти...");
    let mut exit = String::new();
    let _ = io::stdin().read_line(&mut exit);
}

fn check_winner(board: &[[char; 3]; 3]) -> Option<char> {
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
