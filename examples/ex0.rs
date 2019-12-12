fn main() {
    //it might be a good idea to perform terminal-menu stuff
    //in separate functions from other code
    //so that you can use the following line without much confusion:
    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![

        //run the example and try these out
        scroll_selection("Scroll Selection", vec!["Foo", "Bar", "Baz"]),
        scroll_numeric("Scroll Numeric", 0.0, 0.5, -5.0, 10.0),
        list_selection("List Selection", vec!["First", "Second", "Third"]),
        list_numeric("List Numeric", 0.0, 1.0, 0.0, 5.0),
        button("Exit")

    ]);

    println!("(use arrow keys or wasd)");

    //open the menu
    activate(&menu);

    //other work can be done here

    //wait for the menu to exit
    wait_for_exit(&menu);

    //read values
    println!("Scroll Selection: {}", selection_value(&menu, "Scroll Selection").unwrap());
    println!("List Selection: {}", selection_value(&menu, "List Selection").unwrap());
    println!("Scroll Numeric: {}", numeric_value(&menu, "Scroll Numeric").unwrap());
    println!("List Numeric: {}", numeric_value(&menu, "List Numeric").unwrap());
}