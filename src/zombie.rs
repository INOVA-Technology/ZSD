use std::cmp::min;
use std::ops::RangeInclusive;

enum ZombieType {
    Basic,
}

pub struct Zombie {
    health: u64,
    kind: ZombieType,
    attack_power: RangeInclusive<u64>,
}

impl Zombie {
    pub fn new() -> Zombie {
        Zombie {
            health: 10,
            kind: ZombieType::Basic,
            attack_power: 4..=5,
        }
    }

    pub fn take_damage(&mut self, dmg: u64) -> u64 {
        let dmg_taken = min(dmg, self.health);
        self.health -= dmg_taken;
        dmg_taken
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
