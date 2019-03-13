use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        let input = buffer.trim();

        // Do somethin'

        buffer.clear();
    }
}
