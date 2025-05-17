use core::fmt;
use std::{fmt::write, ops::Index};

use rand::prelude::*;
use crate::makao;

use crate::cards::Card;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub max_size: usize,
    pub selected: usize,
}

impl fmt::Display for Hand {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            for card in self.cards.clone() {
                print!("\x1b[44m\x1b[37m🮖🮖\x1b[49m ");
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
                write!(f, "")
            } else {
                if self.max_size != 0 {
                    println!("MaxSize: {}", self.max_size);
                }
                for card in self.cards.clone() {
                    print!("{} ", card);
                }
                write!(f, "")
            }
        }
    }
}

impl Hand {
    pub fn new(size: usize) -> Self {
        return Self {
            cards: vec!(),
            max_size: size,
            selected: 0
        }
    }

    pub fn new_from_vec(vec: Vec<Card>, size: usize) -> Self {
        return Self {
            cards: vec,
            max_size: size,
            selected: 0
        }
    }

    pub fn draw(&mut self, card: Card) -> bool {
        if self.cards.len() != self.max_size || self.max_size == 0 {
            self.cards.push(card);
            return true;
        } else {
            println!("to many cards");
            return false;
        }
    }

    pub fn remove(&mut self, card: Card) {
        let index = self.cards.iter().position(|r| *r == card);
        match index {
            Some(i) => {
                self.cards.remove(i);
            },
            _ => {}
        }
    }

    pub fn draw_from_hand(&mut self, other: &mut Hand, remove: bool) {
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

    pub fn top(self) -> Card {
        if self.cards.len() > 0 {
            self.cards.last().unwrap().to_owned()
        } else {
            Card::new("null", "null", false)
        }
    }

    pub fn round(&mut self, stack: &mut Hand, dealer: &mut Hand) {
        for card in self.cards.clone() {
            if makao::can_play(stack.clone(), card.clone()) {
                self.remove(card.clone());
                stack.draw(card);
                return;
            }
        }
        self.draw_from_hand(dealer, true);
    }
}
