use std::{
    io::{
        stdout,
        Write
    }
};
use crossterm::{
    execute,
    cursor,
    terminal,
    input::{
        input,
        InputEvent,
        KeyEvent,
    },
    screen::RawScreen,
};

pub trait TerminalMenuItem {
    fn left(&mut self);
    fn right(&mut self); //____ to return or to not
    fn enter(&mut self) -> bool;
    fn print(&self);
}

impl TerminalMenuItem for () {
    fn left(&mut self) { }
    fn right(&mut self) { }
    fn enter(&mut self) -> bool {
        true
    }
    fn print(&self) { }
}

pub struct NumericTerminalMenuItem {
    val: f64,
    min: f64,
    step: f64,
    max: f64
}
impl NumericTerminalMenuItem {
    pub fn new(default: f64, min: f64, step: f64, max: f64) -> NumericTerminalMenuItem {
        NumericTerminalMenuItem {
            val: default,
            min,
            step,
            max
        }
    }
}
impl TerminalMenuItem for NumericTerminalMenuItem {
    fn left(&mut self) {
        self.val -= self.step;
        if self.val < self.min {
            self.val = self.min;
        }
    }
    fn right(&mut self) {
        self.val += self.step;
        if self.val > self.max {
            self.val = self.max;
        }
    }
    fn enter(&mut self) -> bool {
        false
    }
    fn print(&self) {
        print!("{}", self.val);
    }
}

pub struct MultiSelectTerminalMenuItem {
    vals: Vec<String>,
    selected: usize
}
impl MultiSelectTerminalMenuItem {
    pub fn new(vals: Vec<String>) -> MultiSelectTerminalMenuItem {
        if vals.len() == 0 {
            panic!("'vals' cannot be empty");
        }
        MultiSelectTerminalMenuItem {
            vals,
            selected: 0
        }
    }
}
impl TerminalMenuItem for MultiSelectTerminalMenuItem {
    fn left(&mut self) {
        if self.selected == 0 {
            self.selected = self.vals.len() - 1;
        }
        else {
            self.selected -= 1;
        }
    }
    fn right(&mut self) {
        if self.selected == self.vals.len() - 1 {
            self.selected = 0;
        }
        else {
            self.selected += 1;
        }
    }
    fn enter(&mut self) -> bool {
        self.right();
        false
    }
    fn print(&self) {
        print!("{}", self.vals[self.selected]);
    }
}

fn offset_y(x: u16, offset_y: i16) {
    execute!(stdout(), cursor::MoveTo(x, (cursor::position().unwrap().1 as i16 + offset_y) as u16)).unwrap();
}
fn move_to_x(x: u16) {
    execute!(stdout(), cursor::MoveTo(x, cursor::position().unwrap().1)).unwrap();
}

fn update_value(
    items: &mut Vec<(String, Box<dyn TerminalMenuItem>)>,
    selected: usize,
    longest_name: u16,
    fun: fn(&str, &dyn TerminalMenuItem, usize) -> ()) {

    offset_y(longest_name + 7, (selected as i16) - (items.len() as i16) + 1);
    execute!(stdout(), terminal::Clear(terminal::ClearType::UntilNewLine)).unwrap();
    items[selected].1.print();
    offset_y(0, (items.len() as i16) - (selected as i16) - 1);
    fun(&items[selected].0, items[selected].1.as_ref(), selected);
}

pub fn display(
    mut items: &mut Vec<(String, Box<dyn TerminalMenuItem>)>,
    fun: fn(&str, &dyn TerminalMenuItem, usize) -> (),
    clear: bool) -> usize {

    if items.len() == 0 {
        panic!("'items' cannot be empty");
    }

    //into raw mode
    let _raw = RawScreen::into_raw_mode();
    execute!(stdout(), cursor::Hide).unwrap();

    //get longest name length for alignment
    let mut longest_name: u16 = 0;
    for (name, _) in &*items {
        if name.len() as u16 > longest_name {
            longest_name = name.len() as u16;
        }
    }

    //print initial stuff
    print!("> {}", items[0].0);
    move_to_x(longest_name + 7);
    items[0].1.print();
    for i in 1..items.len() {
        println!();
        move_to_x(2);
        print!("{}", items[i].0);
        move_to_x(longest_name + 7);
        items[i].1.print();
    }
    move_to_x(0);

    //crossterm stuff
    let input = input();
    let mut sync_stdin = input.read_sync();

    let mut selected: usize = 0;

    loop {
        if let Some(InputEvent::Keyboard(k)) = sync_stdin.next() {
            match k {
                KeyEvent::Up    | KeyEvent::Char('w') => {
                    offset_y(0, (selected as i16) - (items.len() as i16) + 1);
                    print!(" ");
                    if selected == 0 {
                        selected = items.len() - 1;
                        offset_y(0, selected as i16);
                    }
                    else {
                        selected -= 1;
                        offset_y(0, -1);
                    }
                    print!(">");
                    offset_y(0, (items.len() as i16) - (selected as i16) - 1);
                }
                KeyEvent::Down  | KeyEvent::Char('s') => {
                    offset_y(0, (selected as i16) - (items.len() as i16) + 1);
                    print!(" ");
                    if selected == items.len() - 1 {
                        selected = 0;
                        offset_y(0, -(items.len() as i16) + 1);
                    }
                    else {
                        selected += 1;
                        offset_y(0, 1);
                    }
                    print!(">");
                    offset_y(0, (items.len() as i16) - (selected as i16) - 1);
                }

                KeyEvent::Left  | KeyEvent::Char('a') => {
                    items[selected].1.left();
                    update_value(&mut items, selected, longest_name, fun);
                },
                KeyEvent::Right | KeyEvent::Char('d') => {
                    items[selected].1.right();
                    update_value(&mut items, selected, longest_name, fun);
                },
                KeyEvent::Enter => {
                    if items[selected].1.enter() {
                        break;
                    }
                    update_value(&mut items, selected, longest_name, fun);
                }
                _ => {}
            }
        }
    }

    if clear {
        execute!(stdout(),
            cursor::MoveUp(items.len() as u16 - 1),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        ).unwrap();
    }
    else {
        println!();
    }

    execute!(stdout(), cursor::Show).unwrap();
    selected
}
