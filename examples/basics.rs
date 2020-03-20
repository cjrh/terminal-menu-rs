fn main() {

    //it might be a good idea to perform terminal-menu stuff
    //in separate functions from other code
    //so that you can use the following line without much confusion:
    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![

        //run the example and try these out
        scroll("Selection", vec!["First Option", "Second Option", "Third Option"]),
        list("Do Something", vec!["Yes", "No"]),
        numeric("Numeric", 2.75, 0.25, -7.25, 11.5),
        submenu("Submenu", vec![
            scroll("Something", vec!["Select", "From", "These", "Or This"]),
            list("Another", vec!["Foo", "Bar", "Baz"]),
            numeric("Number", 50.0, 1.0, 0.0, 100.0),
            button("Back")
        ]),
        button("Exit")

    ]);

    println!("(use arrow keys or wasd)");

    //open the menu
    activate(&menu);

    //other work can be done here

    //wait for the menu to exit
    wait_for_exit(&menu);

    /*
    //read values
    println!("Selection: {}", selection_value(&menu, "Selection"));
    println!("Do Something: {}", selection_value(&menu, "Do Something"));
    println!("Numeric: {}", numeric_value(&menu, "Numeric"));

    let submenu = get_submenu(&menu, "Submenu");
    println!("Submenu.Something: {}", selection_value(&submenu, "Something"));
    println!("Submenu.Another: {}", selection_value(&submenu, "Another"));
    println!("Submenu.Number: {}", numeric_value(&submenu, "Number"));
    */
}