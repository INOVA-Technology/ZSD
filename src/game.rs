use crate::player::Player;
use crate::zombie::{Zombie, ZombieType};

pub struct Game<I> {
    player: Player,
    current_zombie: Zombie,
    waves: I,
    current_wave: &'static ZombieType,
}

impl<I: Iterator<Item=&'static ZombieType>> Game<I> {
    pub fn new(mut waves: I) -> Game<I> {
        let current_wave = waves.next().unwrap();
        let current_zombie = current_wave.make_zombie();
        Game {
            player: Player::new(),
            current_zombie,
            waves,
            current_wave,
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
        self.current_zombie = self.current_wave.make_zombie();
    }

    pub fn punch(&mut self) {
        let dmg = self.player.punch();
        let dmg_taken = self.current_zombie.take_damage(dmg);
        println!("The zombie took {} damage!", dmg_taken);
    }
}
