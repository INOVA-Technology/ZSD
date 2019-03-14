use crate::player::Player;
use crate::zombie::Zombie;

pub struct Game {
    player: Player,
    current_zombie: Zombie,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(),
            current_zombie: Zombie::new(),
        }
    }
}
