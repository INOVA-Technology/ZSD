use std::cmp;

use rand::{Rng, thread_rng};

use crate::combo::Combo;

pub struct Player {
    health: u64,
    xp: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: 20,
            xp: 0,
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

    pub fn give_xp(&mut self, xp: u64) {
        self.xp += xp;
    }

    pub fn take_xp(&mut self, xp: u64) {
        let xp_taken = cmp::min(self.xp, xp);
        self.xp -= xp_taken;
    }
}
