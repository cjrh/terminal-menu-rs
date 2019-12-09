use std::{
    io::{
        stdout,
        Write
    }
};
use crossterm::{
    execute,
    terminal,
    cursor,
};

pub struct TerminalMenuItem {
    pub name: String,
    pub values: Vec<String>,
    pub selected: usize
}
impl TerminalMenuItem {
    pub fn new(name: &str, values: Vec<&str>) -> TerminalMenuItem {
        TerminalMenuItem {
            name: name.to_owned(),
            values: values.iter().map(|&s| s.to_owned()).collect(),
            selected: 0
        }
    }
}

pub struct TerminalMenu {
    pub items: Vec<TerminalMenuItem>,
    pub selected: usize
}
impl TerminalMenu {
    pub fn new(items: Vec<TerminalMenuItem>) -> TerminalMenu {
        TerminalMenu {
            items,
            selected: 0
        }
    }
    pub fn print(&self) {

        //calc longest name
        let mut longest_name = 0;
        for item in &self.items {
            if item.name.len() > longest_name {
                longest_name = item.name.len();
            }
        }

        let mut i = 0;
        for item in &self.items {
            print!("{} {}",
                if i == self.selected {'>'} else {' '},
                item.name
            );
            if item.values.len() == 0 {
                println!();
            }
            else {
                move_to_x(longest_name as u16 + 6);
                println!("{}", item.values[item.selected]);
            }
            i += 1;
        }

    }
    pub fn erase(&self) {
        execute!(stdout(),
            cursor::MoveUp(self.items.len() as u16),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        ).unwrap();
    }
    pub fn refresh(&self) {
        self.erase();
        self.print();
    }
}

//helper functions
fn move_to_x(x: u16) {
    execute!(stdout(), cursor::MoveTo(x, cursor::position().unwrap().1)).unwrap();
}