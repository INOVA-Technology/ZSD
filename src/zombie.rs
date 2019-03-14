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
}
