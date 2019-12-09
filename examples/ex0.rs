use terminal_menu::{TerminalMenu, TerminalMenuItem};

fn main() {
    let mut tm = TerminalMenu::new(vec![
        TerminalMenuItem::new("Test", vec!["First", "Second"]),
        TerminalMenuItem::new("test2", vec!["Primary", "Secondary"]),
        TerminalMenuItem::new("Exit", vec![])
    ]);

    tm.print();
    tm.selected = 1;
    tm.refresh();
}