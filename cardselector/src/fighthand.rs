use core::fmt;
use std::{fmt::write, ops::Index};

use rand::prelude::*;
use crate::fight;
use crate::fightslot::FightSlot;

use crate::fightcards::FightCard;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FightHand {
    pub cards: Vec<FightCard>,
    pub slots: Vec<FightSlot>,
    pub max_size: usize,
    pub selected: usize,
    pub health: usize
}

impl fmt::Display for FightHand {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            for slot in  self.slots.clone() {
                print!("{} ", slot);
            } 
            print!(" {}\n", self.health);
            for card in self.cards.clone() {
                print!("\x1b[44m\x1b[37m🮖🮖🮖\x1b[49m ");
            }
            write!(f, "")
        } else {
            if self.selected != 0 {
                let card = self.cards.index(self.selected-1);
                if self.max_size != 0 {
                    println!("MaxSize: {} | Selected: {}", self.max_size, card);
                } else {
                    println!("Selected: {}", card);
                }
                for card in self.cards.clone() {
                    print!("{} ", card);
                }
                print!("\n");
                for slot in  self.slots.clone() {
                    print!("{} ", slot);
                } 
                write!(f, "")
            } else {
                if self.max_size != 0 {
                    println!("MaxSize: {}", self.max_size);
                }
                for card in self.cards.clone() {
                    print!("{} ", card);
                }
                print!(" {}\n", self.health);
                for slot in  self.slots.clone() {
                    print!("{} ", slot);
                } 
                write!(f, "")
            }
        }
    }
}

impl FightHand {
    pub fn new(size: usize) -> Self {
        return Self {
            cards: vec!(),
            slots: vec!(FightSlot::new(1), FightSlot::new(2), FightSlot::new(3), FightSlot::new(4)),
            max_size: size,
            selected: 0,
            health: 10
        }
    }

    pub fn new_from_vec(vec: Vec<FightCard>, size: usize) -> Self {
        return Self {
            cards: vec,
            slots: vec!(FightSlot::new(1), FightSlot::new(2), FightSlot::new(3), FightSlot::new(4)),
            max_size: size,
            selected: 0,
            health: 10
        }
    }

    pub fn draw(&mut self, card: FightCard) -> bool {
        if self.cards.len() != self.max_size || self.max_size == 0 {
            self.cards.push(card);
            return true;
        } else {
            println!("to many cards");
            return false;
        }
    }

    pub fn remove(&mut self, card: FightCard) {
        let index = self.cards.iter().position(|r| *r == card);
        match index {
            Some(i) => {
                self.cards.remove(i);
            },
            _ => {}
        }
    }

    pub fn draw_from_hand(&mut self, other: &mut FightHand, remove: bool) {
        if other.cards.len() == 0 {
            println!("Can't draw from empty hand");
        } else if self.cards.len() != self.max_size || self.max_size == 0 {
            let mut rng = rand::rng();
            let car = other.cards.choose(&mut rng).unwrap().clone();
            let drawn = self.draw(car.clone());
            if remove {
                if drawn {
                    other.remove(car.clone());
                }            
            }
        } else {
            println!("to many cards");
        }
    } 

    pub fn move_right(&mut self) {
        if self.selected == self.cards.len() {
            return
        }
        self.selected += 1;
    }

    pub fn move_left(&mut self) {
        if self.selected == 0 {
            return;
        }
        self.selected -= 1;
    }

    pub fn top(self) -> FightCard {
        if self.cards.len() > 0 {
            self.cards.last().unwrap().to_owned()
        } else {
            FightCard::new("null", 0, 0)
        }
    }

    pub fn move_to_slot(&mut self, slot: usize) {
        if slot <= self.slots.len() {
            let card = self.cards[self.selected.clone()-1].clone();
            let changed = self.slots[slot-1].change_card(card.clone());
            if changed {
                self.remove(card);
            }
        }
    }

    pub fn round(&mut self, dealer: &mut FightHand) {
        for card in self.cards.clone() {
            if fight::can_play(card.clone()) {
                self.remove(card.clone());
                // place
                return;
            }
        }
        self.draw_from_hand(dealer, true);
    }
}
