use std::io::{self, Write};

use zombie_smack_down::game::Game;

fn main() {
    let mut buffer = String::new();

    let mut game = Game::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        buffer.make_ascii_lowercase();
        let input = buffer.trim();

        match input {
            "kick" | "k" => game.kick(),
            "punch" | "p" => game.punch(),
            "" => (),
            _ => println!("Invalid command"),
        }

        buffer.clear();
    }
}
