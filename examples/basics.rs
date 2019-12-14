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
        numeric("Numeric", -5.25, 0.25, -10.0, 5.0),
        submenu("Submenu", vec![
            list("Another", vec!["Foo", "Bar", "Baz"]),
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

    //read values
    println!("Selection: {}", selection_value(&menu, "Selection").unwrap());
    println!("Do Something: {}", selection_value(&menu, "Do Something").unwrap());
    println!("Numeric: {}", numeric_value(&menu, "Numeric").unwrap());
    println!("Another: {}",
             selection_value(
                 &get_submenu(&menu, "Submenu").unwrap(),
                 "Another")
             .unwrap()
    );
}