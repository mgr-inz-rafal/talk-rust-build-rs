use crossterm_cursor::cursor;
use crossterm_terminal::terminal;
use std::f64;
use std::{thread, time};

fn main() {
    let cursor = cursor();
    let _ = cursor.hide();
    let terminal = terminal();
    let _ = terminal.clear(crossterm_terminal::ClearType::All);
    println!("Terminal width: {}", terminal.terminal_size().0);

    include!(concat!(env!("OUT_DIR"), "/beret.rs"));

    loop {
        tab.iter().for_each(|v| {
            let _ = cursor.goto(*v, 10);
            print!(" █ ");
            thread::sleep(time::Duration::from_millis(5));
        })
    }
}
