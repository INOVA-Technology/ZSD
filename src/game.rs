use std::process::exit;

use crate::player::Player;
use crate::zombie::{Zombie, ZombieType};

const ZOMBIES_PER_WAVE: u64 = 3;

pub struct Game<I> {
    player: Player,
    current_zombie: Zombie,
    waves: I,
    current_wave: &'static ZombieType,
    zombies_remaining_in_wave: u64,
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
            zombies_remaining_in_wave: ZOMBIES_PER_WAVE - 1,
        }
    }

    pub fn kick(&mut self) {
        self.attack_zombie(self.player.kick());
    }

    pub fn punch(&mut self) {
        self.attack_zombie(self.player.punch());
    }

    fn attack_zombie(&mut self, dmg: u64) {
        let dmg_taken = self.current_zombie.take_damage(dmg);
        println!("The zombie took {} damage!", dmg_taken);
        if !self.current_zombie.is_alive() {
            self.ko();
        }
    }

    fn ko(&mut self) {
        println!("KO!");
        self.next_zombie();
    }

    fn next_zombie(&mut self) {
        if self.zombies_remaining_in_wave == 0 {
            self.next_wave();
        } else {
            self.current_zombie = self.current_wave.make_zombie();
            self.zombies_remaining_in_wave -= 1;
        }
    }

    fn next_wave(&mut self) {
        match self.waves.next() {
            Some(wave) => {
                self.current_wave = wave;
                self.zombies_remaining_in_wave = ZOMBIES_PER_WAVE;
            },
            None => {
                self.win_game();
            }
        }
    }

    fn win_game(&self) {
        println!("You win! Nice");
        // TODO: don't exit here, but rather make a game status and set it to `Win` or something
        exit(0);
    }
}
