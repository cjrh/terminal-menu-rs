use std::thread::sleep;
use std::time::Duration;

fn main() {
    use terminal_menu::*;

    let menu = menu(vec![
        selection("Selection", vec!["First", "Second", "Third"]),
        numeric("Numeric", 0.0, 0.5, -5.0, 10.0),
        button("Exit")
    ]);

    activate(menu.clone());

    while menu.read().unwrap().active {
        sleep(Duration::from_millis(100));
    }
}