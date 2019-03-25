use rustyline::Editor;

use zombie_smack_down::combo::Combo;
use zombie_smack_down::game::Game;
use zombie_smack_down::zombie::WAVES;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut game = Game::new(WAVES.iter().cloned());

    loop {
        let mut input = rl.readline("> ");
        let input = match input {
            Err(_) => break,
            Ok(ref mut input) => {
                rl.add_history_entry(&input[..]);
                input.make_ascii_lowercase();
                input.trim()
            },
        };

        match input {
            "kick" | "k" => game.kick(),
            "punch" | "p" => game.punch(),
            "" => (),
            _ => {
                if let Some(c) = Combo::from_name(input) {
                    game.combo(c);
                } else {
                    println!("Invalid command");
                }
            }
        }
    }
}
