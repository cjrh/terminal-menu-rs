fn main() {
    //it might be a good idea to perform terminal-menu stuff
    //in separate functions from other code
    //so that you can use the following line without much confusion:
    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![

        //run the example and try these out
        scroll("Scroll", vec!["First", "Second", "Third"]),
        list("List", vec!["Foo", "Bar", "Baz"]),
        numeric("Numeric", -5.25, 0.25, -10.0, 5.0),
        button("Exit")

    ]);

    println!("(use arrow keys or wasd)");

    //open the menu
    activate(&menu);

    //other work can be done here

    //wait for the menu to exit
    wait_for_exit(&menu);

    //read values
    println!("Scroll: {}", selection_value(&menu, "Scroll").unwrap());
    println!("List: {}", selection_value(&menu, "List").unwrap());
    println!("Numeric: {}", numeric_value(&menu, "Numeric").unwrap());
}