use crate::FightCard::FightCard;

use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FightSlot {
    id: u8,
    card: Option<FightCard>
}

impl fmt::Display for FightSlot {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Ressult {
        if self.card.is_some() {
            write!(f, "{}", Some(self.card))
        } else {
            write!(f, "sus")
        }
    }
}
