use std::process::exit;

use crate::color;
use crate::combo::Combo;
use crate::player::Player;
use crate::zombie::{Zombie, ZombieType};

const ZOMBIES_PER_WAVE: u64 = 3;

pub struct Game<I> {
    player: Player,
    current_zombie: Zombie,
    waves: I,
    current_wave: ZombieType,
    zombies_remaining_in_wave: u64,
}

impl<I: Iterator<Item=ZombieType>> Game<I> {
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

    pub fn combo(&mut self, combo: Combo) {
        if let Some(dmg) = self.player.combo(combo) {
            println!("You did a sick combo.");
            self.attack_zombie(dmg)
        } else {
            println!(color!(Yellow, "Not enough xp..."));
        }
    }

    fn attack_zombie(&mut self, dmg: u64) {
        let dmg_taken = self.current_zombie.take_damage(dmg);
        println!(color!(Red, "The zombie took {} damage!"), dmg_taken);
        if !self.current_zombie.is_alive() {
            self.next_zombie();
        }
    }

    fn next_zombie(&mut self) {
        println!(color!(Red, "KO!"));
        if self.zombies_remaining_in_wave == 0 {
            if let Some(wave) = self.waves.next() {
                self.current_wave = wave;
                self.zombies_remaining_in_wave = ZOMBIES_PER_WAVE;
            } else {
                println!(color!(Cyan, "You win! Nice"));
                // TODO: don't exit here, but rather make a game status and set it to `Win` or something
                exit(0);
            }
        } else {
            self.current_zombie = self.current_wave.make_zombie();
            self.zombies_remaining_in_wave -= 1;
        }
    }
}
