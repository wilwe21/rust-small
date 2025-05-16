use crate::cards::Card;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub max_size: usize,
    pub selected: u8,
}

impl Hand {
    pub fn new(size: usize) -> Self {
        return Self {
            cards: vec!(),
            max_size: size,
            selected: 0
        }
    }

    pub fn draw(&mut self, card: Card) {
        if self.cards.len() != self.max_size {
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
}
