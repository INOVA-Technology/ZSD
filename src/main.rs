use std::io::{self, Write};

use zombie_smack_down::game::Game;

fn main() {
    let mut buffer = String::new();

    let mut game = Game::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        let input = buffer.trim();

        // Do somethin'

        buffer.clear();
    }
}
