use std::process::exit;

use crate::{println_combat, println_warn, println_levelup};
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
        self.do_combat(self.player.kick());
    }

    pub fn punch(&mut self) {
        self.do_combat(self.player.punch());
    }

    pub fn combo(&mut self, combo: Combo) {
        if let Some(dmg) = self.player.combo(combo) {
            println!("You did a sick combo.");
            self.do_combat(dmg)
        } else {
            println_warn!("Not enough xp...");
        }
    }

    fn do_combat(&mut self, dmg: u64) {
        self.current_zombie.take_damage(dmg);
        println_combat!("The zombie took {} damage!", dmg);
        if self.current_zombie.is_alive() {
            let dmg_taken = self.current_zombie.attack();
            self.player.take_damage(dmg_taken);
            println_combat!("The zombie hit you! -{} hp", dmg_taken);
            if !self.player.is_alive() {
                println_combat!("Oh no! You died... Game over.");
                exit(1);
            }
        } else {
            self.next_zombie();
        }
    }

    fn next_zombie(&mut self) {
        println_combat!("KO!");
        if self.zombies_remaining_in_wave == 0 {
            if let Some(wave) = self.waves.next() {
                self.current_wave = wave;
                self.zombies_remaining_in_wave = ZOMBIES_PER_WAVE;
            } else {
                println_levelup!("You win! Nice");
                // TODO: don't exit here, but rather make a game status and set it to `Win` or something
                exit(0);
            }
        } else {
            self.current_zombie = self.current_wave.make_zombie();
            self.zombies_remaining_in_wave -= 1;
        }
    }
}
