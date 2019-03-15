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

    pub fn kick(&mut self) {
        let dmg = self.player.kick();
        let dmg_taken = self.current_zombie.take_damage(dmg);
        println!("The zombie took {} damage!", dmg_taken);
        if !self.current_zombie.is_alive() {
            self.ko();
        }
    }

    fn ko(&mut self) {
        println!("KO!");
        self.current_zombie = Zombie::new();
    }
}
