use std::cmp::min;
use std::ops::Range;

use rand::{Rng, thread_rng};

#[derive(Clone)]
pub struct ZombieType {
    health: Range<u64>,
    attack_power: Range<u64>,
    name: &'static str,
}

impl ZombieType {
    pub fn make_zombie(&self) -> Zombie {
        Zombie {
            health: thread_rng().gen_range(self.health.start, self.health.end),
            type_name: self.name,
            attack_power: self.attack_power.clone(),
        }
    }
}

pub const WAVES: &'static [ZombieType] = &[
    ZombieType {
        health: 8..11,
        attack_power: 4..6,
        name: "basic",
    },
];

pub struct Zombie {
    health: u64,
    type_name: &'static str,
    attack_power: Range<u64>,
}

impl Zombie {
    pub fn take_damage(&mut self, dmg: u64) -> u64 {
        let dmg_taken = min(dmg, self.health);
        self.health -= dmg_taken;
        dmg_taken
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
