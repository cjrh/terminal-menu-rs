///
/// Explains how submenus work.
///

fn main() {

    use terminal_menu::*;

    //menus inside menus is easy
    let menu = menu(vec![

        label("Submenus"),

        submenu("My Submenu", vec![
            label("Inside My Submenu"),
            list("Option", vec!["Foo", "Bar", "Baz"]),
            back_button("Back"),
        ]),

        submenu("Your Submenu", vec![
            label("Inside Your Submenu"),
            numeric("Option", 3.4, None, None, None),
            button("Exit all menus"),
        ]),

    ]);

    run(&menu);

    //pull value from inside the submenu
    println!("{}", mut_menu())
}