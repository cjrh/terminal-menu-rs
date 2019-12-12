//! Create simple menus for the terminal!
//!
//! [Examples](https://gitlab.com/xamn/terminal-menu-rs/tree/master/examples)

use std::{
    sync::{Arc, RwLock},
    thread,
    io::{stdout, Write},
    time::Duration
};
use crossterm::{
    execute,
    cursor,
    terminal,
    screen::RawScreen,
    input::{input, InputEvent, KeyEvent}
};

type TerminalMenu = Arc<RwLock<TerminalMenuStruct>>;

#[derive(Eq, PartialEq)]
enum TMIKind {
    Button,
    ScrollSelection,
    ListSelection,
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
    n_max: f64
}
/// Make a button terminal-menu item.
/// # Example
/// ```
/// let my_button = terminal_menu::button("My Button");
/// ```
pub fn button(name: &str) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::Button,
        s_values: vec![],
        s_selected: 0,
        n_value: 0.0,
        n_step: 0.0,
        n_min: 0.0,
        n_max: 0.0,
    }
}
/// Make a terminal-menu item from which you can scroll
/// a value from a selection.
/// # Example
/// ```
/// let my_selection = terminal_menu::selection("My Selection", vec![
///     "First Option",
///     "Second Option",
///     "Third Option",
/// ]);
/// ```
pub fn scroll_selection(name: &str, values: Vec<&str>) -> TerminalMenuItem {
    if values.len() == 0 {
        panic!("values cannot be empty");
    }
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::ScrollSelection,
        s_values: values.iter().map(|&s| s.to_owned()).collect(),
        s_selected: 0,
        n_value: 0.0,
        n_step: 0.0,
        n_min: 0.0,
        n_max: 0.0,
    }
}
/// Make a terminal-menu item from which you can select
/// a value from a list.
/// # Example
/// ```
/// let my_selection = terminal_menu::selection("My Selection", vec![
///     "First Option",
///     "Second Option",
///     "Third Option",
/// ]);
/// ```
pub fn list_selection(name: &str, values: Vec<&str>) -> TerminalMenuItem {
    if values.len() == 0 {
        panic!("values cannot be empty");
    }
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::ListSelection,
        s_values: values.iter().map(|&s| s.to_owned()).collect(),
        s_selected: 0,
        n_value: 0.0,
        n_step: 0.0,
        n_min: 0.0,
        n_max: 0.0,
    }
}
/// Make a terminal-menu item from which you can
/// select a number.
/// # Example
/// ```
/// let my_numeric = terminal_menu::numeric("My Numeric",
///     0.0,  //default
///     0.5,  //step
///     -5.0, //minimum
///     10.0  //maximum
/// );
/// ```
pub fn numeric(name: &str, default: f64, step: f64, min: f64, max: f64) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::Numeric,
        s_values: vec![],
        s_selected: 0,
        n_value: default,
        n_step: step,
        n_min: min,
        n_max: max,
    }
}

pub struct TerminalMenuStruct {
    items: Vec<TerminalMenuItem>,
    selected: usize,
    active: bool,
    exited: bool,
}
impl TerminalMenuStruct {
    /// Returns true if the menu is active (open).
    /// # Example
    /// ```
    /// let is_active = menu.read().unwrap().is_active();
    /// ```
    pub fn is_active(&self) -> bool {
        !self.exited
    }
    /// Returns the name of the selected menu item.
    /// # Example
    /// ```
    /// let selected = menu.read().unwrap().selected_item();
    /// ```
    pub fn selected_item(&self) -> &str {
        &self.items[self.selected].name
    }
    /// Returns the value of the specified selection item.
    /// # Example
    /// ```
    /// let s_value = menu.read().unwrap().selection_value("My Selection");
    /// ```
    pub fn selection_value(&self, name: &str) -> Option<&str> {
        for item in &self.items {
            if    (item.kind == TMIKind::ListSelection
                || item.kind == TMIKind::ScrollSelection)
                && item.name.eq(name) {
                return Some(&item.s_values[item.s_selected]);
            }
        }
        None
    }
    /// Returns the value of the specified numeric item.
    /// # Example
    /// ```
    /// let n_value = menu.read().unwrap().numeric_value("My Numeric");
    /// ```
    pub fn numeric_value(&self, name: &str) -> Option<f64> {
        for item in &self.items {
            if item.kind == TMIKind::Numeric && item.name.eq(name) {
                return Some(item.n_value);
            }
        }
        None
    }
}
/// Create a new terminal-menu.
/// # Example
/// ```
/// //create the menu
/// let menu = terminal_menu::menu(vec![
///
///     //lets you select from a list of values with arrow keys
///     terminal_menu::selection("Selection", vec!["First", "Second", "Third"]),
///
///     //lets you select a number with arrow keys
///     //arguments: default, step, min, max
///     terminal_menu::numeric("Numeric", 0.0, 0.5, -5.0, 10.0),
///
///     //buttons when pressed exit the menu
///     terminal_menu::button("Exit")
///
/// ]);
/// ```
pub fn menu(items: Vec<TerminalMenuItem>) -> TerminalMenu {
    if items.len() == 0 {
        panic!("items cannot be empty");
    }
    Arc::new(RwLock::new(TerminalMenuStruct {
        items,
        selected: 0,
        active: false,
        exited: true
    }))
}
/// Shortcut to getting the selected item as a String.
/// # Example
/// ```
/// let selected = terminal_menu::selected_item(&menu);
/// ```
pub fn selected_item(menu: &TerminalMenu) -> String {
    menu.read().unwrap().selected_item().to_owned()
}
/// Shortcut to getting the value of the specified selection item as a String.
/// # Example
/// ```
/// let s_value = terminal_menu::selection_value(&menu, "Selection");
/// ```
pub fn selection_value(menu: &TerminalMenu, item: &str) -> Option<String> {
    menu.read().unwrap().selection_value(item).map(|s| s.to_owned())
}
/// Shortcut to getting the value of the specified numeric item.
/// # Example
/// ```
/// let s_value = terminal_menu::selection_value(&menu, "Selection");
/// ```
pub fn numeric_value(menu: &TerminalMenu, item: &str) -> Option<f64> {
    menu.read().unwrap().numeric_value(item)
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
fn move_left(a: u16) {
    if a != 0 {
        execute!(stdout(), cursor::MoveLeft(a)).unwrap();
    }
}
fn move_right(a: u16) {
    if a != 0 {
        execute!(stdout(), cursor::MoveRight(a)).unwrap();
    }
}
fn move_to_beginning() {
    for _ in 0..terminal::size().unwrap().0 {
        print!("\u{8}");
    }
}
fn clear_rest_of_line() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::UntilNewLine)).unwrap();
}
fn save_pos() {
    execute!(stdout(), cursor::SavePosition).unwrap();
}
fn restore_pos() {
    execute!(stdout(), cursor::RestorePosition).unwrap();
}
fn print_menu(menu: &TerminalMenuStruct, longest_name: usize, selected: usize) {
    for i in 0..menu.items.len() {
        print!("{} {}    ", if i == selected { '>' } else { ' ' }, menu.items[i].name);
        for _ in menu.items[i].name.len()..longest_name {
            print!(" ");
        }
        match menu.items[i].kind {
            TMIKind::Button => {},
            TMIKind::ScrollSelection => print!("{}", menu.items[i].s_values[menu.items[i].s_selected]),
            TMIKind::ListSelection => {
                move_left(1);
                for j in 0..menu.items[i].s_values.len() {
                    print!("{}{}{}",
                           if j == menu.items[i].s_selected {'['} else {' '},
                           menu.items[i].s_values[j],
                           if j == menu.items[i].s_selected {']'} else {' '},
                    );
                }
            }
            TMIKind::Numeric => print!("{}", menu.items[i].n_value)
        }
        if i != menu.items.len() - 1 {
            println!();
        } else {
            move_to_beginning();
            stdout().flush().unwrap();
        }
    }
}
fn run_menu(menu: TerminalMenu) {

    //set active
    {
        let mut menu = menu.write().unwrap();
        menu.active = true;
        menu.exited = false;
    }

    execute!(stdout(), cursor::Hide).unwrap();

    //print initially
    let mut longest_name = 0;
    {
        let menu = menu.read().unwrap();
        for item in &menu.items {
            if item.name.len() > longest_name {
                longest_name = item.name.len();
            }
        }
        print_menu(&menu, longest_name, menu.selected);
    }

    let _raw = RawScreen::into_raw_mode().unwrap();
    let input = input();
    let mut stdin = input.read_async();

    use KeyEvent::*;

    while menu.read().unwrap().active {
        if let Some(InputEvent::Keyboard(k)) = stdin.next() {
            match k {
                Up | Char('w') => {
                    let mut menu = menu.write().unwrap();

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
                    let mut menu = menu.write().unwrap();

                    save_pos();
                    move_up((menu.items.len() - menu.selected - 1) as u16);
                    print!(" ");

                    if menu.selected == menu.items.len() - 1 {
                        menu.selected = 0;
                        move_up(menu.items.len() as u16 - 1);
                    }
                    else {
                        menu.selected += 1;
                        move_down(1);
                    }
                    print!("\u{8}>");

                    restore_pos();
                }
                Left | Char('a') => {
                    let mut menu = menu.write().unwrap();
                    let s = menu.selected;

                    save_pos();
                    move_up((menu.items.len() - s - 1) as u16);
                    move_right(longest_name as u16 + 6);
                    clear_rest_of_line();

                    match menu.items[s].kind {
                        TMIKind::Button => {}
                        TMIKind::ScrollSelection => {
                            if menu.items[s].s_selected == 0 {
                                menu.items[s].s_selected =
                                    menu.items[s].s_values.len() - 1;
                            }
                            else {
                                menu.items[s].s_selected -= 1;
                            }
                            print!("{}", menu.items[s].s_values[
                                menu.items[s].s_selected
                            ]);
                        }
                        TMIKind::ListSelection => {
                            if menu.items[s].s_selected == 0 {
                                menu.items[s].s_selected =
                                    menu.items[s].s_values.len() - 1;
                            }
                            else {
                                menu.items[s].s_selected -= 1;
                            }
                            move_left(1);
                            for i in 0..menu.items[s].s_values.len() {
                                print!("{}{}{}",
                                    if i == menu.items[s].s_selected {'['} else {' '},
                                    menu.items[s].s_values[i],
                                    if i == menu.items[s].s_selected {']'} else {' '},
                                );
                            }
                        }
                        TMIKind::Numeric => {
                            menu.items[s].n_value -=
                                menu.items[s].n_step;
                            if menu.items[s].n_value <
                                menu.items[s].n_min {
                                menu.items[s].n_value =
                                    menu.items[s].n_min;
                            }
                            print!("{}", menu.items[s].n_value);
                        }
                    }

                    restore_pos();
                }
                Right | Char('d') => {
                    let mut menu = menu.write().unwrap();
                    let s = menu.selected;

                    save_pos();
                    move_up((menu.items.len() - s - 1) as u16);
                    move_right(longest_name as u16 + 6);
                    clear_rest_of_line();

                    match menu.items[s].kind {
                        TMIKind::Button => {}
                        TMIKind::ScrollSelection => {
                            if menu.items[s].s_selected ==
                                menu.items[s].s_values.len() - 1 {
                                menu.items[s].s_selected = 0;
                            }
                            else {
                                menu.items[s].s_selected += 1;
                            }
                            print!("{}", menu.items[s].s_values[
                                menu.items[s].s_selected
                            ]);
                        }
                        TMIKind::ListSelection => {
                            if menu.items[s].s_selected ==
                                menu.items[s].s_values.len() - 1 {
                                menu.items[s].s_selected = 0;
                            }
                            else {
                                menu.items[s].s_selected += 1;
                            }
                            move_left(1);
                            for i in 0..menu.items[s].s_values.len() {
                                print!("{}{}{}",
                                       if i == menu.items[s].s_selected {'['} else {' '},
                                       menu.items[s].s_values[i],
                                       if i == menu.items[s].s_selected {']'} else {' '},
                                );
                            }
                        }
                        TMIKind::Numeric => {
                            menu.items[s].n_value +=
                                menu.items[s].n_step;
                            if menu.items[s].n_value >
                                menu.items[s].n_max {
                                menu.items[s].n_value =
                                    menu.items[s].n_max;
                            }
                            print!("{}", menu.items[s].n_value);
                        }
                    }

                    restore_pos();
                }
                Enter | Char(' ') => {
                    let mut menu = menu.write().unwrap();
                    let s = menu.selected;

                    match menu.items[s].kind {
                        TMIKind::Button => {
                            menu.active = false;
                        }
                        TMIKind::ScrollSelection => {
                            save_pos();
                            move_up((menu.items.len() - s - 1) as u16);
                            move_right(longest_name as u16 + 6);
                            clear_rest_of_line();

                            if menu.items[s].s_selected ==
                                menu.items[s].s_values.len() - 1 {
                                menu.items[s].s_selected = 0;
                            } else {
                                menu.items[s].s_selected += 1;
                            }
                            print!("{}", menu.items[s].s_values[
                                menu.items[s].s_selected
                            ]);

                            restore_pos();
                        }
                        TMIKind::ListSelection => {
                            save_pos();
                            move_up((menu.items.len() - s - 1) as u16);
                            move_right(longest_name as u16 + 6);
                            clear_rest_of_line();

                            if menu.items[s].s_selected ==
                                menu.items[s].s_values.len() - 1 {
                                menu.items[s].s_selected = 0;
                            }
                            else {
                                menu.items[s].s_selected += 1;
                            }
                            move_left(1);
                            for i in 0..menu.items[s].s_values.len() {
                                print!("{}{}{}",
                                       if i == menu.items[s].s_selected {'['} else {' '},
                                       menu.items[s].s_values[i],
                                       if i == menu.items[s].s_selected {']'} else {' '},
                                );
                            }

                            restore_pos();
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }
        thread::sleep(Duration::from_millis(10));
    }

    execute!(stdout(),
            cursor::MoveUp(menu.read().unwrap().items.len() as u16 - 1),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Show
        ).unwrap();
    menu.write().unwrap().exited = true;
}

/// Activate (open) the menu.
/// Menus will deactivate when button items are pressed or
/// deactivated manually.
/// # Example
/// ```
/// terminal_menu::activate(&menu);
/// ```
pub fn activate(menu: &TerminalMenu) {
    let menu = menu.clone();
    thread::spawn(move || {
        run_menu(menu);
    });
}
/// Deactivate (exit) a menu manually.
/// # Example
/// ```
/// terminal_menu::deactivate(&menu);
/// ```
pub fn deactivate(menu: &TerminalMenu) {
    menu.write().unwrap().active = false;
    wait_for_exit(menu);
}
/// Wait for menu to deactivate (exit).
/// # Example
/// ```
/// terminal_menu::wait_for_exit(&menu);
/// ```
pub fn wait_for_exit(menu: &TerminalMenu) {
    loop {
        thread::sleep(Duration::from_millis(10));
        if menu.read().unwrap().exited {
            break;
        }
    }
}
/// Activate the menu and wait for it to deactivate (exit).
/// # Example
/// ```
/// terminal_menu::run(&menu);
/// ```
pub fn run(menu: &TerminalMenu) {
    run_menu(menu.clone());
}