///
/// Simple menu with three options.
///

fn main() {
    use terminal_menu::{menu, button, run, mut_menu};

    let menu = menu(vec![
        button("Option 1"),
        button("Option 2"),
        button("Option 3")
    ]);
    run(&menu);
    println!("Selected option: {}", mut_menu(&menu).selected_item_name());
}