#[allow(unused_must_use)]
use crossterm_cursor::cursor;
use crossterm_terminal::terminal;

fn main() {
    let cursor = cursor();
    let terminal = terminal();

    let _ = terminal.clear(crossterm_terminal::ClearType::All);
    let _ = cursor.goto(10, 5);
    println!("Hello, world!");
}
