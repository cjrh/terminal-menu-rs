///
/// Explains every terminal-menu item kinds.
///

fn main() {

    //useful when creating a menu
    use terminal_menu::*;

    //create the menu
    let my_menu = menu(vec![

        //each terminal-menu item kind has it's own example file

        label("--------------------------"),
        label("Use arrow keys or wasd"),
        label("Enter to use, esc to quit!"),
        label("--------------------------"),

        list("My List", vec!["First", "Second", "Third"]),
        scroll("My Scroll", vec!["Foo", "Bar"]),
        string("My String", "Default", false),
        numeric("My Numeric", 0.0, Some(0.5), Some(-10.0), Some(5.0)),

        button("Exit")
    ]);

    //display the menu and wait for exit
    run(&my_menu);

    {
        //get a mutable instance
        //works when menu is exited

        let mm = mut_menu(&my_menu);

        //pull values
        println!("{}", mm.selection_value("My List"));
        println!("{}", mm.selection_value("My Scroll"));
        println!("{}", mm.selection_value("My String"));
        println!("{}", mm.numeric_value("My Numeric"));

        //name of the item which was selected on exit
        println!("{}", mm.selected_item_name());
    }
}