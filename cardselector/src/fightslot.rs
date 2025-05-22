use crate::fightcards::FightCard;

use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FightSlot {
    id: u8,
    card: Option<FightCard>
}

impl fmt::Display for FightSlot {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        if self.card.is_some() {
            write!(f, "{}", self.card.clone().unwrap())
        } else {
            write!(f, "[{}]", self.id)
        }
    }
}

impl FightSlot {
    pub fn new(id: u8) -> Self {
        return Self {
            id,
            card: None
        }
    }

    pub fn change_card(&mut self, card: FightCard) -> bool {
        if !self.card.is_some() {
            self.card = Some(card);
            return true;
        }
        return false;
    }
}
