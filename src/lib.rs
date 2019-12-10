use std::{
    sync::{Arc, RwLock},
    thread,
    io::{
        stdout,
        Write
    }
};
use crossterm::{
    execute,
    cursor,
    terminal,
    screen::RawScreen,
    input::{input, InputEvent, KeyEvent}
};

enum TMIKind {
    Button,
    Selection,
    Numeric,
}
pub struct TerminalMenuItem {
    name: String,
    kind: TMIKind,
    s_values: Vec<String>,
    s_selected: usize,
    n_value: f64,
    n_step: f64,
    n_min: f64,
    n_max: f64,
}
pub fn button(name: &str) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::Button,
        s_values: vec![],
        s_selected: 0,
        n_value: 0.0,
        n_step: 0.0,
        n_min: 0.0,
        n_max: 0.0
    }
}
pub fn selection(name: &str, values: Vec<&str>) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::Selection,
        s_values: values.iter().map(|&s| s.to_owned()).collect(),
        s_selected: 0,
        n_value: 0.0,
        n_step: 0.0,
        n_min: 0.0,
        n_max: 0.0
    }
}
pub fn numeric(name: &str, default: f64, step: f64, min: f64, max: f64) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::Numeric,
        s_values: vec![],
        s_selected: 0,
        n_value: default,
        n_step: step,
        n_min: min,
        n_max: max
    }
}

pub struct TerminalMenu {
    items: Vec<TerminalMenuItem>,
    selected: usize
}
pub fn menu(items: Vec<TerminalMenuItem>) -> TerminalMenu {
   TerminalMenu {
        items,
        selected: 0
    }
}

//helper functions
fn move_up(a: u16) {
    if a != 0 {
        execute!(stdout(), cursor::MoveUp(a)).unwrap();
    }
}
fn move_down(a: u16) {
    if a != 0 {
        execute!(stdout(), cursor::MoveDown(a)).unwrap();
    }
}
fn move_to_beginning() {
    for _ in 0..terminal::size().unwrap().0 {
        print!("\u{8}");
    }
}
fn save_pos() {
    execute!(stdout(), cursor::SavePosition).unwrap();
}
fn restore_pos() {
    execute!(stdout(), cursor::RestorePosition).unwrap();
}

pub fn activate(mut menu: TerminalMenu) {
    thread::spawn(move || {

        execute!(stdout(), cursor::Hide).unwrap();

        let mut longest_name = 0;
        for item in &menu.items {
            if item.name.len() > longest_name {
                longest_name = item.name.len();
            }
        }

        for i in 0..menu.items.len() {
            print!("{} {}    ", if i == 0 {'>'} else {' '}, menu.items[i].name);
            for _ in menu.items[i].name.len()..longest_name {
                print!(" ");
            }
            match menu.items[i].kind {
                TMIKind::Button =>    {},
                TMIKind::Selection => print!("{}", menu.items[i].s_values[menu.items[i].s_selected]),
                TMIKind::Numeric =>   print!("{}", menu.items[i].n_value)
            }
            if i != menu.items.len() - 1 {
                println!();
            }
            else {
                move_to_beginning();
                stdout().flush();
            }
        }

        let raw = RawScreen::into_raw_mode().unwrap();
        let input = input();
        let mut stdin = input.read_sync();

        use KeyEvent::*;

        loop {
            if let Some(InputEvent::Keyboard(k)) = stdin.next() {
                match k {
                    Up | Char('w') => {
                        save_pos();
                        move_up((menu.items.len() - menu.selected - 1) as u16);
                        print!(" ");

                        if menu.selected == 0 {
                            menu.selected = menu.items.len() - 1;
                            move_down(menu.items.len() as u16 - 1);
                        }
                        else {
                            menu.selected -= 1;
                            move_up(1);
                        }
                        print!("\u{8}>");

                        restore_pos();
                    }
                    Down | Char('s') => {
                        if menu.selected == menu.items.len() - 1 {
                            menu.selected = 0;
                        }
                        else {
                            menu.selected += 1;
                        }
                    }
                    _ => ()
                }
            }
        }

        execute!(stdout(), cursor::Show).unwrap();
    });
}