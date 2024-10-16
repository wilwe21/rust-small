use gtk::prelude::*;

use yaml_rust::{YamlLoader, Yaml};
use std::fs;
use std::fs::File;
use home::home_dir;
use std::io::prelude::*;

use gtk::gdk;
use meval::eval_str;
use regex::Regex;

// css classes :90
fn get_conf() {
    match home_dir() {
        Some(h) => {
            let path = format!("{}/.config/wcalc/config.yaml", h.display());
            match File::open(&path) {
                Ok(f) => {
                    println!("File Found")
                },
                _ => {
                    match fs::create_dir(format!("{}/.config/wcalc", h.display())) {
                        Ok(()) => println!("path created"),
                        _ => println!("path not")
                    }
                    let mut f = File::create(path).expect("can't create file");
                    match f.write(b"theme: 'default'") {
                        Ok(_) => println!("write"),
                        _ => println!("not write")
                    }
                    println!("{:?}", f);
                }
            }
        },
        None => println!("no home path find")
    }
}
fn config() {
    get_conf();
    let styles = ["default", "gumball"];
    let con = gtk::Window::builder()
        .title("Wcalc Config")
        .build();
    let bob = gtk::Box::new(gtk::Orientation::Vertical, 0);
    for styl in styles {
        let sty = gtk::Label::builder()
            .label(styl)
            .build();
       bob.append(&sty); 
    }
    con.set_child(Some(&bob));
    con.show();
}

fn on_activate(app: &gtk::Application) {
    let mainBox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let entry = gtk::Entry::builder()
        .has_frame(true)
        .placeholder_text("9+10=21")
        .build();
    let buttonBox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    mainBox.append(&entry);
    mainBox.append(&buttonBox);
    let topbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let midbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let botbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let bbbbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let entupbut = gtk::Box::new(gtk::Orientation::Vertical, 1);
    let parbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let clrbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let porootbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    let button1 = gtk::Button::builder()
        .label("1")
        .build();
    let button2 = gtk::Button::builder()
        .label("2")
        .build();
    let button3 = gtk::Button::builder()
        .label("3")
        .build();
    let buttonminus = gtk::Button::builder()
        .label("-")
        .build();
    let button4 = gtk::Button::builder()
        .label("4")
        .build();
    let button5 = gtk::Button::builder()
        .label("5")
        .build();
    let button6 = gtk::Button::builder()
        .label("6")
        .build();
    let buttonmult = gtk::Button::builder()
        .label("*")
        .build();
    let button7 = gtk::Button::builder()
        .label("7")
        .build();
    let button8 = gtk::Button::builder()
        .label("8")
        .build();
    let button9 = gtk::Button::builder()
        .label("9")
        .build();
    let buttondiv = gtk::Button::builder()
        .label("/")
        .build();
    let button0 = gtk::Button::builder()
        .label("0")
        .build();
    let buttondot = gtk::Button::builder()
        .label(".")
        .build();
    let buttonplus = gtk::Button::builder()
        .label("+")
        .build();
    let buttonexp = gtk::Button::builder()
        .label("^")
        .build();
    let buttonent = gtk::Button::builder()
        .label("=")
        .build();
    let buttonclr = gtk::Button::builder()
        .label("Clr")
        .build();
    let buttonlefpar = gtk::Button::builder()
        .label("(")
        .build();
    let buttonrigpar = gtk::Button::builder()
        .label(")")
        .build();
    let buttonrmlas = gtk::Button::builder()
        .label("Del")
        .build();
    let buttonroot = gtk::Button::builder()
        .label("√")
        .build();
    // css related
    topbut.add_css_class("topbut");
    midbut.add_css_class("midbut");
    botbut.add_css_class("botbut");
    bbbbut.add_css_class("bbbbut");
    entupbut.add_css_class("entupbut");
    parbox.add_css_class("parbox");
    clrbox.add_css_class("clrbox");
    porootbox.add_css_class("porootbox");
    buttonent.add_css_class("enter");
    entry.connect_activate(|entry| {
        let text = entry.text();
        if text == "conf" {
            config();
            entry.set_text("");
        } else {
            match meval::eval_str(text.clone()) {
                Ok(vyl) => {
                    let end = vyl.to_string();
                    entry.set_text(&end);
                    return
                } Err(e) => {
                    let chpar = |strin: &str| {
                        let one = "Parse error: Missing 1 right parenthesis.";
                        let par = Regex::new(r"Parse error: Missing .* right parentheses.")
                            .map(|regex| {
                                if regex.is_match(&strin.clone()) || &strin == &one {
                                    let numb = strin.to_string().chars()
                                        .find(|a| a.is_digit(10))
                                        .and_then(|a| a.to_digit(10)).unwrap() as usize;
                                    let pars = ")".repeat(numb);
                                    let mut ccc = format!("{}{}", text.clone(), pars);
                                    match meval::eval_str(ccc.clone()) {
                                        Ok(vyl) => {
                                            let end = vyl.to_string();
                                            entry.set_text(&end);
                                            entry.set_position(end.len() as i32);
                                            return 
                                        } Err(ee) => {
                                            println!("Error pars ee: {}", ee);
                                            entry.set_text(&ccc);
                                            entry.set_position(ccc.len() as i32);
                                            return
                                        }
                                    }
                                }
                            })
                            .unwrap_or_else(|error| {
                                println!("error");
                            });
                    };
                    let chtok = |strin: &str| {
                        let untok = Regex::new(r"Parse error: Unexpected token at byte .*")
                            .map(|regex| {
                                if regex.is_match(&strin) {
                                    let ccc = text.clone();
                                    let pos = strin.to_string().chars()
                                        .find(|a| a.is_digit(10))
                                        .and_then(|a| a.to_digit(10)).unwrap() as i32;
                                    let test = |exp: String, pos: i32| {
                                        let mut st = exp;
                                        st.insert(pos.try_into().unwrap(), "*".chars().next().unwrap());
                                        match meval::eval_str(st.clone()) {
                                            Ok(vyl) => {
                                                let end = vyl.to_string();
                                                entry.set_text(&end);
                                                entry.set_position(end.len() as i32);
                                                return [st, end];
                                            } Err(e) => {
                                                entry.set_text(&st);
                                                entry.set_position(st.len() as i32);
                                                return [st, e.to_string()];
                                            }
                                        }
                                    };
                                    let mut stra = test(ccc.clone().to_string(), pos);
                                    let mut iter = 0;
                                    loop {
                                        if regex.is_match(&stra[1]) && iter > 20 {
                                            let pos = stra[1].to_string().chars()
                                                .find(|a| a.is_digit(10))
                                                .and_then(|a| a.to_digit(10)).unwrap() as i32;
                                            stra = test(stra[0].clone().to_string(), pos);
                                            iter += 1;
                                        } else {
                                            break
                                        }
                                    }
                                    return
                                }
                            })
                            .unwrap_or_else(|error| {
                                println!("error");
                            });
                    };
                    chpar(&e.to_string());
                    chtok(&e.to_string());
                    println!("Error: {}", e);
                }
        }}
    });
    let butclick = move |button: &gtk::Button| {
        let sas = button.label().unwrap();
        let cur = entry.text();
        if sas == "Clr" {
            entry.set_text("");
            return
        }
        if sas == "Del" {
            if cur == "" {
                return
            }
            let new = &cur[..cur.len() -1];
            entry.set_text(new);
            return
        }
        if sas != "=" {
            let mut endst = String::new();
            if sas == "√" {
                endst = format!("{}^(1/",&cur);
            } else {
                endst = format!("{}{}",&cur, &sas);
            }
            entry.set_text(&endst);
            return
        }
        if sas == "=" {
            if cur == "" {
                return
            } else if cur == "conf" {
                config();
                entry.set_text("");
            } else {
                match meval::eval_str(cur.clone()) {
                    Ok(vyl) => {
                        let end = vyl.to_string();
                        entry.set_text(&end);
                        return
                    } Err(e) => {
                        let chpar = |strin: &str| {
                            let one = "Parse error: Missing 1 right parenthesis.";
                            let par = Regex::new(r"Parse error: Missing .* right parentheses.")
                                .map(|regex| {
                                    if regex.is_match(&strin.clone()) || &strin == &one {
                                        let numb = strin.to_string().chars()
                                            .find(|a| a.is_digit(10))
                                            .and_then(|a| a.to_digit(10)).unwrap() as usize;
                                        let pars = ")".repeat(numb);
                                        let mut ccc = format!("{}{}", cur.clone(), pars);
                                        match meval::eval_str(ccc.clone()) {
                                            Ok(vyl) => {
                                                let end = vyl.to_string();
                                                entry.set_text(&end);
                                                entry.set_position(end.len() as i32);
                                                return 
                                            } Err(ee) => {
                                                println!("Error pars ee: {}", ee);
                                                entry.set_text(&ccc);
                                                entry.set_position(ccc.len() as i32);
                                                return
                                            }
                                        }
                                    }
                                })
                                .unwrap_or_else(|error| {
                                    println!("error");
                                });
                        };
                        let chtok = |strin: &str| {
                            let untok = Regex::new(r"Parse error: Unexpected token at byte .*")
                                .map(|regex| {
                                    if regex.is_match(&strin) {
                                        let ccc = cur.clone();
                                        let pos = strin.to_string().chars()
                                            .find(|a| a.is_digit(10))
                                            .and_then(|a| a.to_digit(10)).unwrap() as i32;
                                        let test = |exp: String, pos: i32| {
                                            let mut st = exp;
                                            st.insert(pos.try_into().unwrap(), "*".chars().next().unwrap());
                                            match meval::eval_str(st.clone()) {
                                                Ok(vyl) => {
                                                    let end = vyl.to_string();
                                                    entry.set_text(&end);
                                                    entry.set_position(end.len() as i32);
                                                    return [st, end];
                                                } Err(e) => {
                                                    entry.set_text(&st);
                                                    entry.set_position(st.len() as i32);
                                                    return [st, e.to_string()];
                                                }
                                            }
                                        };
                                        let mut stra = test(ccc.clone().to_string(), pos);
                                        let mut iter = 0;
                                        loop {
                                            if regex.is_match(&stra[1]) && iter > 20 {
                                                let pos = stra[1].to_string().chars()
                                                    .find(|a| a.is_digit(10))
                                                    .and_then(|a| a.to_digit(10)).unwrap() as i32;
                                                stra = test(stra[0].clone().to_string(), pos);
                                                iter += 1;
                                            } else {
                                                break
                                            }
                                        }
                                        return
                                    }
                                })
                                .unwrap_or_else(|error| {
                                    println!("error");
                                });
                        };
                        chpar(&e.to_string());
                        chtok(&e.to_string());
                        println!("Error: {}", e);
                    }
                }
            }
        }
    };
    buttonplus.connect_clicked(butclick.clone());
    buttonminus.connect_clicked(butclick.clone());  
    buttondiv.connect_clicked(butclick.clone());
    buttonmult.connect_clicked(butclick.clone());  
    buttondot.connect_clicked(butclick.clone());
    button0.connect_clicked(butclick.clone());
    button1.connect_clicked(butclick.clone());
    button2.connect_clicked(butclick.clone());
    button3.connect_clicked(butclick.clone());
    button4.connect_clicked(butclick.clone());
    button5.connect_clicked(butclick.clone());
    button6.connect_clicked(butclick.clone());
    button7.connect_clicked(butclick.clone());
    button8.connect_clicked(butclick.clone());
    button9.connect_clicked(butclick.clone());
    buttonent.connect_clicked(butclick.clone());
    buttonexp.connect_clicked(butclick.clone());
    buttonclr.connect_clicked(butclick.clone());
    buttonlefpar.connect_clicked(butclick.clone());
    buttonrigpar.connect_clicked(butclick.clone());
    buttonrmlas.connect_clicked(butclick.clone());
    buttonroot.connect_clicked(butclick.clone());
    topbut.append(&button7);
    midbut.append(&button8);
    botbut.append(&button9);
    bbbbut.append(&buttondiv);
    topbut.append(&button4);
    midbut.append(&button5);
    botbut.append(&button6);
    bbbbut.append(&buttonmult);
    topbut.append(&button1);
    midbut.append(&button2);
    botbut.append(&button3);
    bbbbut.append(&buttonminus);
    topbut.append(&button0);
    midbut.append(&buttondot);
    bbbbut.append(&buttonplus);
    parbox.append(&buttonlefpar);
    parbox.append(&buttonrigpar);
    clrbox.append(&buttonclr);
    clrbox.append(&buttonrmlas);
    porootbox.append(&buttonexp);
    porootbox.append(&buttonroot);
    entupbut.append(&porootbox);
    entupbut.append(&parbox);
    entupbut.append(&clrbox);
    entupbut.append(&buttonent);
    buttonBox.append(&topbut);
    buttonBox.append(&midbut);
    buttonBox.append(&botbut);
    buttonBox.append(&bbbbut);
    buttonBox.append(&entupbut);
    let window = gtk::ApplicationWindow::builder()
        .title("WCalc")
        .application(app)
        .build();
    load_css();
    window.set_child(Some(&mainBox));
    window.show();
}

fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    let css_content = include_str!("../css/main.css");
    provider.load_from_data(css_content);
    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.wilwe21.Calc")
        .build();
    app.connect_activate(on_activate);
    app.run();
}
