pub struct squere {
    color: u8,
    text: String,
    text_color: u8,
    h: u16,
    w: u16,
}
impl squere {
    pub fn new(color: u8, text: String, text_color: u8, h:u16, w:u16) -> Self {
        Self {
            color,
            text,
            text_color,
            h,
            w,
        }
    }
    pub fn draw(&self) {
        let h: usize = self.h.into();
        print!("\x1b[48;5;{}m\x1b[38;5;{}m{:^h$}\x1b[0m",self.color,self.text_color,self.text)
    }
}
pub struct frame {
    width: u16,
    height: u16,
    color: u8,
    title: String,
    title_color: u8,
}
impl frame {
    pub fn new(width: u16, height: u16, color: u8, title: String, title_color: u8) -> Self {
        Self {
            width,
            height,
            color,
            title,
            title_color,
        }
    }
    pub fn draw(&self) {
        let w: usize = (self.width).into();
        println!("\x1b[48;5;{}m\x1b[38;5;{}m\x1b[1A", self.color, self.title_color);
        println!("{:^w$}", self.title);
        println!("\x1b[0m\x1b[1A");
        let w2: usize = (self.width - 2).into();
        for i in 0..(self.height-2) {
            println!("\x1b[48;5;{}m \x1b[0m{:^w2$}\x1b[48;5;{}m \x1b[0m",self.color, " ", self.color);
        }
        print!("\x1b[48;5;{}m{:^w$}\x1b[0m\x1b[{}A\x1b[{}D",self.color,"",(self.height-2),(self.width-1));
    }
    pub fn goto_line(&self,line:u16) {
        print!("\x1b[{};2H", (line+2));
    }
    pub fn end(&self) {
        print!("\x1b[{};0H", (self.height+2));
    }
}
