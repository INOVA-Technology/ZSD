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

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn kick(&self) -> u64 {
        thread_rng().gen_range(4, 6)
    }

    pub fn punch(&self) -> u64 {
        thread_rng().gen_range(3, 5)
    }

    pub fn combo(&mut self, combo: Combo) -> Option<u64> {
        if self.xp >= combo.cost {
            self.xp -= combo.cost;
            Some(combo.perform())
        } else {
            None
        }
    }

    pub fn give_xp(&mut self, xp: u64) {
        self.xp += xp;
    }

    pub fn take_xp(&mut self, xp: u64) {
        let xp_taken = cmp::min(self.xp, xp);
        self.xp -= xp_taken;
    }

    pub fn take_damage(&mut self, dmg: u64) {
        self.health -= cmp::min(dmg, self.health);
    }
}
