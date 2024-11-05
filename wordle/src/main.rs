use std::io;
use random_word::*;

fn input(ln: usize) -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp);
    let inp = inp.replace("\n","");
    let inps = inp.chars().collect::<Vec<_>>();
    let mut en = vec!();
    for i in 0..ln {
        en.push(inps[i].to_string())
    }
    println!("\x1b[1A\x1b[K");
    return en.join("")
} 

fn check(inp: String, word: String) -> String {
    let jed: Vec<_> = inp.chars().collect();
    let dwa: Vec<_> = word.chars().collect();
    let mut end = vec!();
    for x in 0..(word.len()) {
        if jed[x] == dwa[x] {
            end.push(format!("\x1b[102m{}\x1b[0m", jed[x]).to_string());
        } else if dwa.contains(&jed[x]) {
            end.push(format!("\x1b[103m{}\x1b[0m",jed[x]).to_string());
        } else {
            end.push(jed[x].to_string());
        }
    }
    return end.join("")
}

fn main() {
    let lang = random_word::Lang::En;
    let wl = 5;
    let word = random_word::gen_len(wl.clone(), lang).unwrap();
    for i in 0..5 {
        let inp = input(wl.clone()).replace("\n", "");
        if inp == word {
            println!("\x1B[1A\x1B[102m{}\x1B[0m", word);
            println!("you won");
            return
        } else {
            let form = check(inp.to_string(), word.to_string());
            println!("\x1B[1A{}", form);
        }
    }
    println!("Correct Word: {}\nYou such a looser", word);
}
