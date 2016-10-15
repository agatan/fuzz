extern crate getopts;
extern crate rustbox;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(name: &str) -> Result<String, io::Error> {
    let mut file = try!(File::open(name));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn print_needle(rbox: &mut rustbox::RustBox, needle: &str) {
    rbox.print(0,
               0,
               rustbox::RB_NORMAL,
               rustbox::Color::White,
               rustbox::Color::Default,
               "QUERY> ");
    rbox.print(7,
               0,
               rustbox::RB_NORMAL,
               rustbox::Color::White,
               rustbox::Color::Default,
               needle);
}

fn print_lines(rbox: &mut rustbox::RustBox, lines: &[&str], selected_line: usize) {
    let width = rbox.width();
    for (i, line) in lines.iter().enumerate() {
        if selected_line == i {
            let line = format!("{:width$}", line, width = width);
            rbox.print(0,
                       i + 1,
                       rustbox::RB_NORMAL,
                       rustbox::Color::Red,
                       rustbox::Color::Cyan,
                       &line);
        } else {
            rbox.print(0,
                       i + 1,
                       rustbox::RB_NORMAL,
                       rustbox::Color::White,
                       rustbox::Color::Default,
                       line);
        }
    }
}

fn search<'a>(src: &[&'a str], needle: &str) -> Vec<&'a str> {
    src.iter().filter(|s| s.contains(needle)).map(|&s| s).collect()
}

fn fuzzy_find(contents: &str) -> Option<&str> {
    let mut rbox = rustbox::RustBox::init(Default::default()).unwrap();
    let lines: Vec<_> = contents.lines().collect();
    let mut current_lines = lines.clone();
    let mut needle = String::new();
    let mut selected_line = 0;
    loop {
        if selected_line >= current_lines.len() && !current_lines.is_empty() {
            selected_line = current_lines.len() - 1;
        }
        rbox.clear();
        print_needle(&mut rbox, &needle);
        print_lines(&mut rbox, &current_lines, selected_line);
        rbox.present();
        match rbox.poll_event(false) {
            Err(e) => panic!("{}", e),
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    rustbox::Key::Esc => {
                        return None;
                    }
                    rustbox::Key::Char(c) => {
                        needle.push(c);
                        current_lines = search(&lines, &needle);
                    }
                    rustbox::Key::Backspace => {
                        needle.pop();
                        current_lines = search(&lines, &needle);
                    }
                    rustbox::Key::Up => {
                        if selected_line > 0 {
                            selected_line -= 1;
                        }
                    }
                    rustbox::Key::Down => {
                        selected_line += 1;
                    }
                    rustbox::Key::Enter => return current_lines.get(selected_line).map(|&s| s),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut opts = getopts::Options::new();
    let matches = opts.parse(&args[1..]).unwrap();

    for filename in matches.free.iter() {
        let contents = read_file(filename).unwrap();
        let result = fuzzy_find(&contents);
        if let Some(res) = result {
            println!("{}", res);
        }
    }
}
