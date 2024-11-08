pub fn draw() {
    let frame_bg = "3";
    let frame_fg = "0";
    let width = 50;
    let height = 30;
    let title = "calc";
    println!("\x1b[48;5;{}m\x1b[38;5;{}m\x1b[1A", frame_bg, frame_fg);
    println!("{:^width$}", title);
    println!("\x1b[0m\x1b[1A");
    let w2 = width - 2;
    for i in 0..(height-2) {
        println!("\x1b[48;5;{}m \x1b[0m{:^w2$}\x1b[48;5;{}m \x1b[0m",frame_bg, " ", frame_bg);
    }
    println!("\x1b[48;5;{}m{:^width$}\x1b[0m",frame_bg,"");
}
