use std::sync::{Arc, RwLock};

fn main() {
    use terminal_menu::*;

    let mut menu = menu(vec![
        selection("Selection", vec!["First", "Second"]),
        numeric("Numeric", 0.0, 0.5, -5.0, 10.0),
        button("Exit")
    ]);

    activate(menu);

    loop {}

}