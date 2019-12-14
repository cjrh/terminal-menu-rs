#[derive(Eq, PartialEq)]
enum TMIKind {
    Button,
    Scroll,
    List,
    Numeric,
    Submenu,
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
    submenu: Option<TerminalMenu>
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
        submenu: None
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
    if values.len() == 0 {
        panic!("values cannot be empty");
    }
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::Scroll,
        s_values: values.iter().map(|&s| s.to_owned()).collect(),
        s_selected: 0,
        n_value: 0.0,
        n_step: 0.0,
        n_min: 0.0,
        n_max: 0.0,
        submenu: None
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
    if values.len() == 0 {
        panic!("values cannot be empty");
    }
    TerminalMenuItem {
        name: name.to_owned(),
        kind: TMIKind::List,
        s_values: values.iter().map(|&s| s.to_owned()).collect(),
        s_selected: 0,
        n_value: 0.0,
        n_step: 0.0,
        n_min: 0.0,
        n_max: 0.0,
        submenu: None
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
        kind: TMIKind::Numeric,
        s_values: vec![],
        s_selected: 0,
        n_value: default,
        n_step: step,
        n_min: min,
        n_max: max,
        submenu: None
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
        kind: TMIKind::Submenu,
        s_values: vec![],
        s_selected: 0,
        n_value: 0.0,
        n_step: 0.0,
        n_min: 0.0,
        n_max: 0.0,
        submenu: Some(menu(items))
    }
}