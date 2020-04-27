//! Display simple menus on the terminal!
//! [Examples](https://gitlab.com/xamn/terminal-menu-rs/tree/master/examples)

#![allow(dead_code)]

mod fancy_menu;
mod basic_menu;
mod utils;

use utils::*;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::thread;
use std::time::Duration;

type TerminalMenu = Arc<RwLock<TerminalMenuStruct>>;

enum TMIKind {
    Label,
    Button,
    BackButton,
    Scroll  { values: Vec<String>, selected: usize },
    List    { values: Vec<String>, selected: usize },
    Numeric { value:  f64, step: Option<f64>, min: Option<f64>, max: Option<f64> },
    Submenu(TerminalMenu),
}
pub struct TerminalMenuItem {
    name: String,
    kind: TMIKind,
    last_print_len: usize
}


/// Make a label terminal-menu item.
/// Can not be selected.
/// Useful for example as a title, separator, or help text.
/// # Example
/// ```
/// let my_button = terminal_menu::button("My Button");
/// ```
pub fn label<T: Into<String>>(text: T) -> TerminalMenuItem {
    TerminalMenuItem {
        name: text.into(),
        kind: TMIKind::Label,
        last_print_len: 0,
    }
}

/// Make a button terminal-menu item.
/// Exits the menu with all the parent menus when pressed.
/// # Example
/// ```
/// let my_button = terminal_menu::button("My Button");
/// ```
pub fn button<T: Into<String>>(name: T) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.into(),
        kind: TMIKind::Button,
        last_print_len: 0,
    }
}

/// Make a button terminal-menu item.
/// Returns to the previous menu (or exits when there is none) when pressed.
/// # Example
/// ```
/// let my_button = terminal_menu::button("My Button");
/// ```
pub fn back_button<T: Into<String>>(name: T) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.into(),
        kind: TMIKind::BackButton,
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
pub fn scroll<T: Into<String>, T2: IntoIterator>(name: T, values: T2) -> TerminalMenuItem where T2::Item: Into<String> {
    let values: Vec<String> = values.into_iter().map(|a| a.into()).collect();
    if values.is_empty() {
        panic!("values cannot be empty");
    }
    TerminalMenuItem {
        name: name.into(),
        kind: TMIKind::Scroll {
            values,
            selected: 0
        },
        last_print_len: 0
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
pub fn list<T: Into<String>, T2: IntoIterator>(name: T, values: T2) -> TerminalMenuItem where T2::Item: Into<String> {
    let values: Vec<String> = values.into_iter().map(|a| a.into()).collect();
    if values.is_empty() {
        panic!("values cannot be empty");
    }
    TerminalMenuItem {
        name: name.into(),
        kind: TMIKind::List {
            values,
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
///     Some(0.5),  //step
///     Some(-5.0), //minimum
///     Some(10.0)  //maximum
/// );
/// ```
pub fn numeric<T: Into<String>>(name: T, default: f64, step: Option<f64>, min: Option<f64>, max: Option<f64>) -> TerminalMenuItem {
    if !utils::value_valid(default, step, min, max) {
        panic!("invalid default value");
    }
    if !utils::step_valid(step, min, max) {
        panic!("invalid step");
    }
    TerminalMenuItem {
        name: name.into(),
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
///     terminal_menu::back_button("Back")
/// ]);
/// ```
pub fn submenu<T: Into<String>>(name: T, items: Vec<TerminalMenuItem>) -> TerminalMenuItem {
    TerminalMenuItem {
        name: name.into(),
        kind: TMIKind::Submenu(menu(items)),
        last_print_len: 0,
    }
}

enum TMStatus {
    Inactive,
    Normal,
    Altscreen { topmost: usize, normal_not_printed: bool, modified: bool }
}

pub struct TerminalMenuStruct {
    pub items: Vec<TerminalMenuItem>,
    selected: usize,
    active: bool,
    exited: bool,

    temporary_menu: Option<TerminalMenu>,
    item_changed: bool,

    longest_name: usize,

    status: TMStatus
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
    /// let selected = mutable_menu_instance.selected_item();
    /// ```
    pub fn selected_item_name(&self) -> &str {
        &self.items[self.selected].name
    }

    /// Returns the index of the selected menu item.
    /// # Example
    /// ```
    /// let selected = mutable_menu_instance.selected_item();
    /// ```
    pub fn selected_item_index(&self) -> usize {
        self.selected
    }

    /// Set the selected item as an index of the items vec
    /// # Example
    /// ```
    /// terminal_menu::mutable_instance(&menu).set_selected_item(5);
    /// ```
    pub fn set_selected_item(&mut self, index: usize) {
        if index >= self.items.len() {
            panic!("index out of range");
        }

    }

    /// Returns the value of the specified selection item.
    /// # Example
    /// ```
    /// let s_value = mutable_menu_instance.selection_value("My Selection");
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
    /// let n_value = mutable_menu_instance.numeric_value("My Numeric");
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
    /// let submenu = mutable_menu_instance.get_submenu("My Submenu");
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
    for i in 0..items.len() {
        if let TMIKind::Label = items[i].kind {
        } else {
            return Arc::new(RwLock::new(TerminalMenuStruct {
                items,
                selected: i,
                active: false,
                exited: true,
                temporary_menu: None,
                item_changed: false,
                longest_name: 0,
                status: TMStatus::Inactive,
            }))
        }
    }
    panic!("no selectable items");
}

/// Shortcut to getting the selected item as a String.
/// # Example
/// ```
/// let selected_name = terminal_menu::selected_item_name(&menu);
/// ```
pub fn selected_item_name(menu: &TerminalMenu) -> String {
    menu.read().unwrap().selected_item_name().to_owned()
}

/// Shortcut to getting the index of the selected item.
/// # Example
/// ```
/// let selected_index = terminal_menu::selected_item_index(&menu);
/// ```
pub fn selected_item_index(menu: &TerminalMenu) -> usize {
    menu.read().unwrap().selected_item_index()
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

/// Shortcut to see if a menu has exited or never been activated.
/// # Example
/// ```
/// if terminal_menu::has_exited(&menu) {
///     ...
/// }
/// ```
pub fn has_exited(menu: &TerminalMenu) -> bool {
    menu.read().unwrap().exited
}

/// Get a mutable instance of the menu.
/// Works only if has_exited(&menu) is true.
/// # Example
/// ```
/// if terminal_menu::has_exited(&menu) {
///     let mut mutable_menu = terminal_menu::mutable_instance(&menu);
///     mutable_menu.set_selected_item(5);
/// }
/// ```
pub fn get_mutable_instance(menu: &TerminalMenu) -> RwLockWriteGuard<TerminalMenuStruct> {
    if !has_exited(menu) {
        panic!("Cannot call mutable_instance if has_exited() is not true");
    }
    menu.write().unwrap()
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
        fancy_menu::run(menu.clone())
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
        if has_exited(menu) {
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
    fancy_menu::run(menu.clone());
}

/// Activate (open) the menu as the basic variant and wait for it to deactivate (exit).
/// Menu will deactivate when deactivated manually or button items are selected.
/// # Example
/// ```
/// terminal_menu::activate_basic(&menu);
/// ```
fn run_basic(menu: &TerminalMenu) {
    basic_menu::run(menu.clone());
}

/// Try to activate (open) the menu as the fancy variant and wait for it to deactivate (exit).
/// returns Err(()) when the terminal does not support it.
/// Menu will deactivate when deactivated manually or button items are pressed.
/// # Example
/// ```
/// terminal_menu::try_run_fancy(&menu);
/// ```
pub fn try_run_fancy(menu: &TerminalMenu) -> Result<(), ()> {
    if !crossterm_compatible() {
        return Err(());
    }
    fancy_menu::run(menu.clone());
    Ok(())
}