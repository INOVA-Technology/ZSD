// use rand::{Rng as _, thread_rng};
use rand::{Rng, thread_rng};

pub struct Player {
    health: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: 20,
        }
    }

    pub fn kick(&self) -> u64 {
        thread_rng().gen_range(4, 6)
    }

    pub fn punch(&self) -> u64 {
        thread_rng().gen_range(3, 5)
    }
}
