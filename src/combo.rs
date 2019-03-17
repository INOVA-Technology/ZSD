use std::ops::Range;

use rand::{Rng, thread_rng};

pub struct Combo {
    power: Range<u64>,
}

impl Combo {
    pub fn from_name(name: &str) -> Option<Combo> {
        let parts = &name.split_whitespace().collect::<Vec<_>>()[..];
        Some(match parts {
            &["kick", "kick", "punch"] => Combo { power: 3..10 },
            _ => return None,
        })
    }

    pub fn perform(&self) -> u64 {
        thread_rng().gen_range(self.power.start, self.power.end)
    }
}
