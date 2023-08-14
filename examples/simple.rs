///
/// A simple menu with three options to choose from.
///

fn main() {
    use terminal_menu::{button, label, menu, mut_menu, run};
    let menu = menu(vec![
        // label:
        //  not selectable, useful as a title, separator, etc...
        label("----------------------"),
        label("terminal-menu"),
        label("use wasd or arrow keys"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------------"),
        // button:
        //  exit the menu
        button("Alice"),
        button("Bob"),
        button("Charlie"),
    ]);
    run(&menu);

    // you can get the selected buttons name like so:
    println!("Selected: {}", mut_menu(&menu).selected_item_name());
}
