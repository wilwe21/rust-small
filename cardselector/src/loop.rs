use std::io;
use std::ops::Index;

use crate::cards::Card;
use crate::hand::Hand;

pub fn gameloop() {
    let path = "/home/wilwe/.config/card/";
    let cards = Card::load_all(&path);
    let mut dealer = Hand::new_from_vec(cards.clone(), 0);
    let mut hand = Hand::new(8);
    hand.draw_from_hand(&mut dealer, true);
    hand.draw_from_hand(&mut dealer, true);
    hand.draw_from_hand(&mut dealer, true);
    loop {
        println!("\x1B[2J");
        println!("\x1B[38;5;3m{}", hand);
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Somethings wrong");
        let user_input = user_input.strip_suffix("\n").unwrap().to_string();
        if ["left".to_string(), "l".to_string()].contains(&user_input.to_lowercase()) {
            hand.move_left();
        } else if ["right".to_string(), "r".to_string()].contains(&user_input.to_lowercase()) {
            hand.move_right();
        } else if ["draw".to_string(), "d".to_string()].contains(&user_input.to_lowercase()) {
            hand.draw_from_hand(&mut dealer, true);
        } else if ["play".to_string(), "p".to_string()].contains(&user_input.to_lowercase()) {
            if hand.selected != 0 {
                hand.remove(hand.cards.index(hand.selected-1).clone());
                if hand.selected > hand.cards.len() {
                    hand.move_left();
                }
            }
        } else if ["q".to_string(), ":q".to_string(), "exit".to_string()].contains(&user_input) {
            break;
        }
    }
}
