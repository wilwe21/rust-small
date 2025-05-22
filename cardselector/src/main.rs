mod hand;
mod cards;
mod fightcards;
mod fighthand;
mod fightslot;
mod fight;
mod makao;

use std::io;

fn main() {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Somethings wrong");
    let user_input = user_input.strip_suffix("\n").unwrap().to_string();
    if ["m".to_string(), "makao".to_string()].contains(&user_input) {
        makao::gameloop("/home/wilwe/.config/card/playing/");
    } else if ["m+".to_string(), "makao+".to_string()].contains(&user_input) {
        makao::gameloop("/home/wilwe/.config/card/playing+/");
    } else if ["f".to_string(), "fight".to_string()].contains(&user_input) {
        fight::gameloop();
    }
}
