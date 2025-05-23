use std::io;
use std::ops::Index;
use std::thread::{sleep};
use std::time::Duration;

use crate::cards::Card;
use crate::hand::Hand;

pub fn gameloop(path: &str) {
    let cards = Card::load_all(path);
    let amount = 8;
    let mut dealer = Hand::new_from_vec(cards.clone(), 0);
    let mut hand = Hand::new(0);
    let mut enemigo = Hand::new(0);
    //let mut enemigo2 = Hand::new(0);
    //let mut enemigo3 = Hand::new(0);
    let mut stack = Hand::new(0);
    let mut players = vec!(hand, enemigo/*, enemigo2, enemigo3*/);
    for p in 0..players.len() {
        for _ in 0..amount { 
            players[p].draw_from_hand(&mut dealer, true);
        }
    }
    stack.draw_from_hand(&mut dealer, true);
    let mut round: usize = 0;
    loop {
        println!("\x1B[2J\x1B[33m");
        print_table(players.clone(), stack.clone());
        if round == 0 {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Somethings wrong");
            let user_input = user_input.strip_suffix("\n").unwrap().to_string();
            if ["left".to_string(), "l".to_string()].contains(&user_input.to_lowercase()) {
                players[0].move_left();
            } else if ["right".to_string(), "r".to_string()].contains(&user_input.to_lowercase()) {
                players[0].move_right();
            } else if ["draw".to_string(), "d".to_string()].contains(&user_input.to_lowercase()) {
                players[0].draw_from_hand(&mut dealer, true);
                round += 1;
            } else if ["play".to_string(), "p".to_string()].contains(&user_input.to_lowercase()) {
                if players[0].selected != 0 {
                    let car = players[0].cards.index(players[0].selected-1).clone();
                    if can_play(stack.clone(), car.clone()) {
                        players[0].remove(car.clone());
                        stack.draw(car);
                        if players[0].selected > players[0].cards.len() {
                            players[0].move_left();
                        }
                        if players[0].cards.len() < 1 {
                            println!("You won");
                            break;
                        }
                        round += 1;
                    }
                }
            } else if ["q".to_string(), ":q".to_string(), "exit".to_string()].contains(&user_input) {
                break;
            }
        } else {
            players[round].round(&mut stack, &mut dealer);
            if players[round].cards.len() < 1 {
                players.remove(round);
                if players.len() < 2 {
                    println!("You loss");
                    break;
                }
            }
            if round == players.len()-1 {
                round = 0;
            } else {
                round += 1;
                sleep(Duration::new(2, 0));
            }
        }
    }
}

pub fn print_table(hands: Vec<Hand>, stack: Hand) {
    let half = hands.clone().len()/2;
    for (i, hand) in hands.iter().enumerate() {
        if i == half {
            println!("{}\n", stack.clone().top());
        }
        if i != 0 {
            println!("{:#}\n", hand);
        } else {
            println!("{}\n", hand);
        }
    }
}

pub fn can_play(stack: Hand, card: Card) -> bool {
    let top = stack.top();
    return top.suit == card.suit || top.value == card.value;
}
