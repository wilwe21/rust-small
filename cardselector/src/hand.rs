use core::fmt;
use std::{fmt::write, ops::Index};

use rand::prelude::*;

use crate::cards::Card;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub max_size: usize,
    pub selected: usize,
}

impl fmt::Display for Hand {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        if self.selected != 0 {
            let card = self.cards.index(self.selected-1);
            println!("MaxSize: {} | Selected: {} {}", self.max_size, self.selected, card);
            for card in self.cards.clone() {
                print!("{} ", card);
            }
            write!(f, "")
        } else {
            println!("MaxSize: {} | Selected: {}", self.max_size, self.selected);
            for card in self.cards.clone() {
                print!("{} ", card);
            }
            write!(f, "")
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

    pub fn draw(&mut self, card: Card) {
        if self.cards.len() != self.max_size || self.max_size == 0 {
            self.cards.push(card);
        } else {
            println!("to many cards");
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
        if self.cards.len() != self.max_size || self.max_size == 0 {
            let mut rng = rand::rng();
            let car = other.cards.choose(&mut rng).unwrap().clone();
            self.draw(car.clone());
            if remove {
                other.remove(car.clone());
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
}
