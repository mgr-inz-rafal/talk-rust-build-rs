use crossterm_cursor::cursor;
use crossterm_terminal::terminal;

fn main() {
    let mut cursor = cursor();
    let mut terminal = terminal();
    terminal.clear(crossterm_terminal::ClearType::All);

    cursor.goto(10, 5);
    println!("Hello, world!");
}
