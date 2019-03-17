use rand::{Rng, thread_rng};

use crate::combo::Combo;

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

    pub fn combo(&self, combo: Combo) -> Option<u64> {
        // TODO: check if there's enough xp, return None if not
        Some(combo.perform())
    }
}
