mod hand;
mod cards;

use crate::cards::Card;
use crate::hand::Hand;

fn main() {
    let AK = Card::new("red", "A", false);
    println!("{:?}", AK);
    let mut hand = Hand::new(8);
    hand.draw(AK.clone());
    println!("{:?}", &hand);
}
