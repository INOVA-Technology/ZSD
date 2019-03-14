pub struct Player {
    health: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: 20,
        }
    }
}
