use std::io::{self, Write as IoWrite};
use std::fmt::Write as FmtWrite;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;
use crossterm::{execute, cursor, terminal, event};
use lazy_static::lazy_static;

const MAX_FLOAT_PRINTING_PRECISION: usize = 10;

lazy_static! {
    pub static ref STDOUT: Mutex<io::Stdout> = Mutex::new(io::stdout());
    pub static ref STDIN: Mutex<io::Stdin> = Mutex::new(io::stdin());
    pub static ref SECOND: Duration = Duration::from_secs(1);
}
pub fn get_stdout() -> MutexGuard<'static, io::Stdout> {
    STDOUT.lock().unwrap()
}

pub fn crossterm_compatible() -> bool {
    return match crossterm::event::poll(Duration::from_nanos(1)) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn move_cursor_left(n: usize) {
    if n != 0 {
        execute!(get_stdout(), cursor::MoveLeft(n as u16)).unwrap();
    }
}
pub fn move_cursor_right(n: usize) {
    if n != 0 {
        execute!(get_stdout(), cursor::MoveRight(n as u16)).unwrap();
    }
}
pub fn move_cursor_up(n: usize) {
    if n != 0 {
        execute!(get_stdout(), cursor::MoveUp(n as u16)).unwrap();
    }
}
pub fn move_cursor_down(n: usize) {
    if n != 0 {
        execute!(get_stdout(), cursor::MoveDown(n as u16)).unwrap();
    }
}
pub fn term_width() -> usize {
    terminal::size().unwrap().0 as usize
}
pub fn term_height() -> usize {
    terminal::size().unwrap().1 as usize
}
pub fn move_cursor_to_row(row: usize) {
    move_cursor_to(0, row);
}
pub fn move_cursor_to_bottom() {
    execute!(get_stdout(), cursor::MoveTo(0, term_height() as u16 - 1)).unwrap();
}
pub fn cursor_column() -> usize {
    cursor::position().unwrap().0 as usize
}
pub fn cursor_row() -> usize {
    cursor::position().unwrap().1 as usize
}
pub fn move_cursor_to(x: usize, y: usize) {
    execute!(get_stdout(), cursor::MoveTo(x as u16, y as u16)).unwrap();
}
pub fn move_cursor_to_column_0() {
    execute!(get_stdout(), cursor::MoveToColumn(0)).unwrap();
}
pub fn save_cursor_pos() {
    execute!(get_stdout(), cursor::SavePosition).unwrap();
}
pub fn restore_cursor_pos() {
    execute!(get_stdout(), cursor::RestorePosition).unwrap();
}
pub fn clear_from_cursor_down() {
    execute!(get_stdout(), terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
}
pub fn clear_current_line() {
    execute!(get_stdout(), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
}
pub fn clear_rest_of_line() {
    execute!(get_stdout(), terminal::Clear(terminal::ClearType::UntilNewLine)).unwrap();
}
pub fn clear_screen() {
    execute!(get_stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}
pub fn cursor_visibility(on: bool) {
    if on {
        execute!(get_stdout(), cursor::Show).unwrap();
    } else {
        execute!(get_stdout(), cursor::Hide).unwrap();
    }
}
pub fn terminal_raw_mode(on: bool) {
    if on {
        terminal::enable_raw_mode().unwrap();
    } else {
        terminal::disable_raw_mode().unwrap();
    }
}
pub fn terminal_alt_screen(on: bool) {
    if on {
        execute!(get_stdout(), terminal::EnterAlternateScreen).unwrap();
    } else {
        execute!(get_stdout(), terminal::LeaveAlternateScreen).unwrap();
    }
}
pub fn cursor_at_bottom_row() -> bool {
    cursor_row() == term_height() - 1
}
pub fn terminal_append_line() {
    if cursor_at_bottom_row() {
        terminal_raw_mode(false);
        println!();
        terminal_raw_mode(true);
    } else {
        move_cursor_down(1);
        move_cursor_to_column_0();
    }
}
pub fn flush() {
    get_stdout().flush().unwrap();
}
pub fn float_printing_precision(n: f64) -> usize {
    let s = n.to_string();
    match s.split('.').nth(1) {
        Some(s) => s.len().min(MAX_FLOAT_PRINTING_PRECISION),
        None => 0
    }
}
pub fn value_valid(value: f64, step: Option<f64>, min: Option<f64>, max: Option<f64>) -> bool {
    if value.is_nan() {
        return false;
    }
    if let Some(min) = min {
        if value < min {
            return false;
        }
    }
    if let Some(max) = max {
        if value > max {
            return false;
        }
    }
    if let Some(step) = step {
        if (value - min.unwrap_or(max.unwrap_or(0.0)).abs()) % step != 0.0 {
            return false;
        }
    }
    true
}
pub fn step_valid(step: Option<f64>, min: Option<f64>, max: Option<f64>) -> bool {
    return if let Some(step) = step {
        if step.is_nan() {
            return false;
        }
        let min = min.unwrap_or(0.0);
        let max = max.unwrap_or(std::f64::MAX);
        step < max - min && (max - min) % step == 0.0
    } else {
        true
    }
}
pub fn number_range_indicator(step: Option<f64>, min: Option<f64>, max: Option<f64>) -> String {
    let mut prefix = String::new();
    if let Some(step) = step {
        if let Some(min) = min {
            write!(prefix, "[{:.*}, {:.*}, ..",
                   float_printing_precision(min), min,
                   float_printing_precision(min + step), min + step,
            ).unwrap();
            if let Some(max) = max {
                write!(prefix, ", {:.*}] ", float_printing_precision(max), max).unwrap();
            } else {
                write!(prefix, "] ").unwrap();
            }
        } else if let Some(max) = max {
            write!(prefix, "[.., {:.*}, {:.*}] ",
                   float_printing_precision(max - step), max - step,
                   float_printing_precision(max), max
            ).unwrap();
        } else {
            write!(prefix, "[.., {:.*}, 0, {:.*}, ..] ",
                   float_printing_precision(-step), -step,
                   float_printing_precision(step), step
            ).unwrap();
        }
    } else if let Some(min) = min {
        if let Some(max) = max {
            write!(prefix, "[{:.*}..{:.*}] ",
                   float_printing_precision(min), min,
                   float_printing_precision(max), max
            ).unwrap();
        } else {
            write!(prefix, "[> {:.*}] ", float_printing_precision(min), min).unwrap();
        }
    } else if let Some(max) = max {
        write!(prefix, "[< {:.*}] ", float_printing_precision(max), max).unwrap();
    } else {
        write!(prefix, ": ").unwrap();
    }
    prefix
}
pub fn number_input(input: &mut String) {
    loop {
        if event::poll(SECOND.clone()).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                use event::KeyCode::*;
                match key_event.code {
                    Enter => break,
                    Char(c) => {
                        if  cursor_column() == term_width() - 1 ||
                            !c.is_ascii_digit() &&
                                (c != '.' || input.contains('.')) &&
                                (c != '-' || !input.is_empty()) {
                            continue;
                        }
                        print!("{}", c);
                        flush();
                        input.push(c);
                    }
                    Backspace => {
                        if !input.is_empty() {
                            print!("\u{8}");
                            clear_rest_of_line();
                            input.truncate(input.len() - 1);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}