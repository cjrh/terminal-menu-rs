//! Create simple menus for the terminal!
//!
//! [Examples](https://gitlab.com/xamn/terminal-menu-rs/tree/master/examples)

mod utils;
use utils::*;

use std::{
    sync::{Arc, RwLock},
    thread,
    io::{stdout, Write as IoWrite},
    time::Duration,
    fmt::Write as FmtWrite
};
use crossterm::{
    execute,
    cursor,
    terminal,
    event
};

type TerminalMenu = Arc<RwLock<TerminalMenuStruct>>;

enum TMIKind {
    Button,
    Scroll  { values: Vec<String>, selected: usize },
    List    { values: Vec<String>, selected: usize },
    Numeric { value:  f64, step: f64, min: f64, max: f64 },
    Submenu(TerminalMenu),
}
pub struct TerminalMenuItem {
    name: String,
    kind: TMIKind,
    last_print_len: usize
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
        last_print_len: 0,
    }
}
/// Make a terminal-menu item from which you can select a value from a selection.
/// # Example
/// ```
/// let my_selection = terminal_menu::scroll("My Selection", vec![
///     "First Option",
///     "Second Option",
///     "Third Option",
/// ]);
/// ```
pub fn scroll(name: &str, values: Vec<&str>) -> TerminalMenuItem {
    if values.is_empty() {
        panic!("values cannot be empty");
    }
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::Scroll {
            values:   values.iter().map(|&s| s.to_owned()).collect(),
            selected: 0
        },
        last_print_len: values[0].len() + 1
    }
}
/// Make a terminal-menu item from which you can select a value from a selection.
/// # Example
/// ```
/// let my_selection = terminal_menu::list("My Selection", vec![
///     "First Option",
///     "Second Option",
///     "Third Option",
/// ]);
/// ```
pub fn list(name: &str, values: Vec<&str>) -> TerminalMenuItem {
    if values.is_empty() {
        panic!("values cannot be empty");
    }
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::List {
            values:   values.iter().map(|&s| s.to_owned()).collect(),
            selected: 0
        },
        last_print_len: 0
    }
}
/// Make a terminal-menu item from which you can select a number between specified bounds.
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
        kind: TMIKind::Numeric {
            value: default,
            step,
            min,
            max
        },
        last_print_len: 0,
    }
}
/// Make a terminal-menu submenu item.
/// # Example
/// ```
/// let my_submenu = terminal_menu::submenu("My Submenu", vec![
///     terminal_menu::list("List", vec!["First", "Second", "Third"]),
///     terminal_menu::button("Back")
/// ]);
/// ```
pub fn submenu(name: &str, items: Vec<TerminalMenuItem>) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::Submenu(menu(items)),
        last_print_len: 0,
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
    pub fn selection_value(&self, name: &str) -> &str {
        for item in &self.items {
            if item.name == name {
                if let TMIKind::List { values, selected } = &item.kind {
                    return &values[*selected];
                }
                if let TMIKind::Scroll { values, selected } = &item.kind {
                    return &values[*selected];
                }
            }
        }
        panic!("Item not found or is wrong kind");
    }
    /// Returns the value of the specified numeric item.
    /// # Example
    /// ```
    /// let n_value = menu.read().unwrap().numeric_value("My Numeric");
    /// ```
    pub fn numeric_value(&self, name: &str) -> f64 {
        for item in &self.items {
            if item.name == name {
                if let TMIKind::Numeric { value, .. } = &item.kind {
                    return *value;
                }
            }
        }
        panic!("Item not found or is wrong kind");
    }
    /// Returns the specified submenu.
    /// # Example
    /// ```
    /// let submenu = menu.read().unwrap().get_submenu("My Submenu");
    /// ```
    pub fn get_submenu(&self, name: &str) -> TerminalMenu {
        for item in &self.items {
            if item.name == name {
                if let TMIKind::Submenu(submenu) = &item.kind {
                    return submenu.clone();
                }
            }
        }
        panic!("Item not found or is wrong kind");
    }
}

/// Create a new terminal-menu.
/// # Example
/// ```
/// let menu = terminal_menu::menu(vec![
///     terminal_menu::list("Do Stuff", vec!["Yes", "No"]),
///     terminal_menu::button("Exit")
/// ]);
/// ```
pub fn menu(items: Vec<TerminalMenuItem>) -> TerminalMenu {
    if items.is_empty() {
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
pub fn selection_value(menu: &TerminalMenu, item: &str) -> String {
    menu.read().unwrap().selection_value(item).to_owned()
}
/// Shortcut to getting the value of the specified numeric item.
/// # Example
/// ```
/// let s_value = terminal_menu::numeric_value(&menu, "Selection");
/// ```
pub fn numeric_value(menu: &TerminalMenu, item: &str) -> f64 {
    menu.read().unwrap().numeric_value(item)
}
/// Shortcut to getting the specified submenu.
/// # Example
/// ```
/// let submenu = terminal_menu::get_submenu(&menu, "Submenu");
/// ```
pub fn get_submenu(menu: &TerminalMenu, item: &str) -> TerminalMenu {
    menu.read().unwrap().get_submenu(item)
}

fn print_menu(menu: &TerminalMenuStruct, longest_name: usize, selected: usize) {
    for i in 0..menu.items.len() {
        if i != 0 {
            println!();
        }
        if i == menu.items.len() - 1 {
            save_cursor_pos();
        }

        //TODO: how many spaces and why?
        print!("{} {}   ", if i == selected { '>' } else { ' ' }, menu.items[i].name);

        for _ in menu.items[i].name.len()..longest_name {
            print!(" ");
        }
        match &menu.items[i].kind {
            TMIKind::Button | TMIKind::Submenu(_) => {}
            TMIKind::Scroll { values, selected } => print!(" {}", values[*selected]),
            TMIKind::List   { values, selected } => {
                for j in 0..values.len() {
                    print!("{}{}{}",
                           if j == *selected {'['} else {' '},
                           values[j],
                           if j == *selected {']'} else {' '},
                    );
                }
            }
            TMIKind::Numeric { value, .. } => print!(" {}", value),
        }
    }
    restore_cursor_pos();
    stdout().lock().flush().unwrap();
}
fn change_active_item(menu: &TerminalMenu, up: bool) {
    let mut menu = menu.write().unwrap();

    save_cursor_pos();
    move_cursor_up(menu.items.len() - menu.selected - 1);
    print!(" ");

    if up {
        if menu.selected == 0 {
            menu.selected = menu.items.len() - 1;
            move_cursor_down(menu.items.len() - 1);
        } else {
            menu.selected -= 1;
            move_cursor_up(1);
        }
    }
    else {
        if menu.selected == menu.items.len() - 1 {
            menu.selected = 0;
            move_cursor_up(menu.items.len() - 1);
        } else {
            menu.selected += 1;
            move_cursor_down(1);
        }
    }

    print!("\u{8}>");

    restore_cursor_pos();
}
#[derive(Eq, PartialEq, Copy, Clone)]
enum Action {
    Left,
    Right,
    Enter
}
fn change_selection_on_selection_item(selected: &mut usize, values: &mut Vec<String>, action: Action) {
    if action == Action::Left {
        if *selected == 0 {
            *selected = values.len() - 1;
        } else {
            *selected -= 1;
        }
    }
    else {
        if *selected == values.len() - 1 {
            *selected = 0;
        } else {
            *selected += 1;
        }
    }
}
fn use_menu_item(menu: &TerminalMenu, longest_name: usize, action: Action) {
    let mut menu = menu.write().unwrap();

    save_cursor_pos();
    move_cursor_up(menu.items.len() - menu.selected - 1);

    //TODO: maybe fix this magic number?
    move_cursor_right(longest_name + 5);

    let _selected = menu.selected;
    let menu_selected_item = &mut menu.items[_selected];
    if let TMIKind::Button | TMIKind::Submenu(_) = &menu_selected_item.kind {
        if action == Action::Enter {
            menu.active = false;
        }
    }
    else {
        let mut print_buf = String::new();
        match &mut menu_selected_item.kind {
            TMIKind::Scroll { selected, values} => {
                change_selection_on_selection_item(selected, values, action);
                write!(print_buf, " {}", values[*selected]).unwrap();
            }
            TMIKind::List { selected, values } => {
                change_selection_on_selection_item(selected, values, action);
                for i in 0..values.len() {
                    write!(print_buf, "{}{}{}",
                           if i == *selected { '[' } else { ' ' },
                           values[i],
                           if i == *selected { ']' } else { ' ' },
                    ).unwrap();
                }
            }
            TMIKind::Numeric { value, step, min, max } => {
                if action == Action::Right {
                    *value += *step;
                    if value > max {
                        *value = *max;
                    }
                } else if action == Action::Left {
                    *value -= *step;
                    if value < min {
                        *value = *min;
                    }
                }
                write!(print_buf, " {}", *value).unwrap();
            }
            _ => panic!("update match above")
        }

        print!("{}", print_buf);

        let new_print_len = print_buf.len();
        for _ in new_print_len..menu_selected_item.last_print_len {
            print!(" ");
        }
        menu_selected_item.last_print_len = new_print_len;
    }

    restore_cursor_pos();
}
fn handle_key_event(menu: &TerminalMenu, longest_name: usize, key_event: event::KeyEvent) {
    use event::KeyCode::*;
    match key_event.code {
        Up | Char('w') => change_active_item(&menu, true),
        Down | Char('s') => change_active_item(&menu, false),
        Left  | Char('a') => use_menu_item(&menu, longest_name, Action::Left),
        Right | Char('d') => use_menu_item(&menu, longest_name, Action::Right),
        Enter | Char(' ') => use_menu_item(&menu, longest_name, Action::Enter),
        _ => {}
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

    terminal::enable_raw_mode().unwrap();

    while menu.read().unwrap().active {
        if event::poll(Duration::from_secs(1)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                handle_key_event(&menu, longest_name, key_event);
            }
        }
    }

    execute!(stdout(),
            cursor::MoveUp(menu.read().unwrap().items.len() as u16 - 1),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Show
    ).unwrap();
    terminal::disable_raw_mode().unwrap();

    let mut runagain = false;
    {
        let menu_rd = menu.read().unwrap();
        if let TMIKind::Submenu(submenu) = &menu_rd.items[menu_rd.selected].kind {
            submenu.write().unwrap().selected = 0;
            run(submenu);
            runagain = true;
        }
    }

    if runagain {
        run(&menu);
    }
    else {
        menu.write().unwrap().exited = true;
    }
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