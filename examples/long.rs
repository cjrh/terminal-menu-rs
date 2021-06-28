///
/// A long menu.
///

fn main() {

    use terminal_menu::*;

    // test out a long list
    let menu = menu(
        (1..100).map(
            |a| button(a.to_string())
        ).collect()
    );

    run(&menu);

    println!("{}", mut_menu(&menu).selected_item_name());
}