pub fn draw() {
    let frame_bg = "3";
    let frame_fg = "0";
    let width = 50;
    let title = "calc";
    println!("\x1b[48;5;{}m\x1b[38;5;{}m\x1b[1A", frame_bg, frame_fg);
    println!("{:^width$}", title);
    println!("\x1b[0m\x1b[1A");
}
