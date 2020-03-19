//! Create simple menus for the terminal!
//!
//! [Examples](https://gitlab.com/xamn/terminal-menu-rs/tree/master/examples)

mod fancy_menu;
mod basic_menu;
mod utils;

use utils::*;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

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

pub fn crossterm_compatible() -> bool {
    return match crossterm::event::poll(Duration::from_nanos(1)) {
        Ok(_) => true,
        Err(_) => false
    }
}
fn run_consuming(menu: TerminalMenu) {
    fancy_menu::run(menu);
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

/// Activate (open) the menu.
/// Menu will deactivate when deactivated manually or button items are pressed.
/// # Example
/// ```
/// terminal_menu::activate(&menu);
/// ```
pub fn activate(menu: &TerminalMenu) {
    let menu = menu.clone();
    thread::spawn(move || {
        run_consuming(menu);
    });
}
/// Activate (open) the menu as the basic variant.
/// Menu will deactivate when deactivated manually or button items are selected.
/// # Example
/// ```
/// terminal_menu::activate_basic(&menu);
/// ```
fn activate_basic(menu: &TerminalMenu) {
    let menu = menu.clone();
    thread::spawn(move || {
        basic_menu::run(menu);
    });
}
/// Try to activate (open) the menu as the fancy variant.
/// returns Err(()) when the terminal does not support it.
/// Menu will deactivate when deactivated manually or button items are pressed.
/// # Example
/// ```
/// terminal_menu::try_activate_fancy(&menu);
/// ```
pub fn try_activate_fancy(menu: &TerminalMenu) -> Result<(), ()> {
    if !crossterm_compatible() {
        return Err(());
    }
    let menu = menu.clone();
    thread::spawn(move || {
        fancy_menu::run(menu);
    });
    Ok(())
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
    run_consuming(menu.clone());
}