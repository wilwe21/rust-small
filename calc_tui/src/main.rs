mod draw;

use crate::draw::*;

fn main() {
    println!("\x1b[2J\x1b[0;0H");
    let w = draw::frame::new(50,30,3,"calc".to_string(),0);
    let but = draw::squere::new(4,"chuj".to_string(), 0, 0,0);
    let butt = draw::squere::new(4,"chuj".to_string(), 0, 0,0);
    w.draw();
    but.draw();
    w.goto_line(2);
    butt.draw();
    w.end();
}
