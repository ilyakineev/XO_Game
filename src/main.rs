mod board;
mod game;
mod utils;

use game::Game;

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
    let _ = std::io::stdin().read_line(&mut exit);
}
