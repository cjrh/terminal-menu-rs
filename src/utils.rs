use std::io::{stdout, Write};
use crossterm::{execute, cursor};

//helper functions
/*pub fn move_cursor_left(n: usize) {
    for _ in 0..n {
        let _ = execute!(stdout(), cursor::MoveLeft(n as u16));
    }
}*/
pub fn move_cursor_right(n: usize) {
    if n != 0 {
        let _ = execute!(stdout(), cursor::MoveRight(n as u16));
    }
}
pub fn move_cursor_up(n: usize) {
    if n != 0 {
        let _ = execute!(stdout(), cursor::MoveUp(n as u16));
    }
}
pub fn move_cursor_down(n: usize) {
    if n != 0 {
        let _ = execute!(stdout(), cursor::MoveDown(n as u16));
    }
}
pub fn save_cursor_pos() {
    execute!(stdout(), cursor::SavePosition).unwrap();
}
pub fn restore_cursor_pos() {
    execute!(stdout(), cursor::RestorePosition).unwrap();
}