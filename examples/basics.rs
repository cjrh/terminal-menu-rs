fn main() {

    //useful when creating a menu
    use terminal_menu::*;

    //create the menu
    let my_menu = menu(vec![

        // the first argument of the terminal-menu item functions is the name
        // which will be displayed. Values are also pulled with this name.

        // label: Title or separator, can not be selected
        label("--------------------------"),
        label("Use arrow keys or wasd"),
        label("Enter to use, esc to quit!"),
        label("--------------------------"),

        // list: display values in a list like so: (selected is in brackets)
        // [value 1]  value 2  value 3
        list("My List", vec!["First", "Second", "Third"]),

        // scroll: scroll through values and display the selected one
        scroll("My Scroll", vec!["Foo", "Bar"]),

        // string: any string of characters
        // set the last param to true if empty strings should be allowed
        string("My String", "Default", false),

        // numeric: select a number, parameters got like so
        // default value, step, minimum, maximum
        numeric("My Numeric", 0.0, Some(0.5), Some(-10.0), Some(5.0)),

        // button: Exit all menus
        button("My Exit")

    ]);

    //display the menu and wait for exit
    run(&my_menu);

    //pull values
    println!("{}", mut_menu(&my_menu).selection_value("My List"));
    println!("{}", mut_menu(&my_menu).selection_value("My Scroll"));
    println!("{}", mut_menu(&my_menu).selection_value("My String"));
    println!("{}", mut_menu(&my_menu).numeric_value("My Numeric"));

    //name of the value which was selected on exit
    println!("{}", mut_menu(&my_menu).selected_item_name());
}