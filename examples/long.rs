///
/// Example of a long list. (run this example yourself)
///

fn main() {
    use terminal_menu::{menu, button, run, mut_menu};
    let mut menu = menu(
        (1..100).map(|n| button(format!("{}", n))).collect()
    );
    run(&menu);
    println!("{}", mut_menu(&menu).selected_item_name());
}