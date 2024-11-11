use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Write};
use std::process::exit;

mod draw;

use crate::draw::*;

fn frame() {
    //println!("\x1b[2J\x1b[0;0H");
    let w = draw::frame::new(50,30,3,"calc".to_string(),0);
    let but = draw::squere::new(4,"chuj".to_string(), 0, 8,0);
    let butt = draw::squere::new(4,"chuj".to_string(), 0, 0,0);
    w.draw();
    but.draw();
    w.goto_line(2);
    butt.draw();
    w.end();
}

fn kill() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    exit(3);
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let enable_key =
        [KeyCode::Char('1'), KeyCode::Char('2'), KeyCode::Char('3'),
        KeyCode::Char('4'), KeyCode::Char('5'), KeyCode::Char('6'),
        KeyCode::Char('7'), KeyCode::Char('8'), KeyCode::Char('9'),
        KeyCode::Char('0'), KeyCode::Char('+'), KeyCode::Char('-'),
        KeyCode::Char('/'), KeyCode::Char('*'), KeyCode::Char('^')];
    enable_raw_mode()?;
    stdout().flush()?;
    println!("working");
    loop {
        stdout().flush()?;
        let event = read()?;
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char('c') => kill()?,
                    _ if enable_key.contains(&key_event.code) => print!("{}", key_event.code),
                    _ => ()
                }
            },
            _ => (),
        }
    }
}
