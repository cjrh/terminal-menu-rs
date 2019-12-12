fn main() {
    //it might be a good idea to perform terminal-menu stuff
    //in separate functions from other code
    //so that you can use the following line without much confusion:
    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![

        //lets you scroll from a list of values with arrow keys or enter
        scroll_selection("Scroll", vec!["First", "Second", "Third"]),

        //same as previous but you see all options at once
        list_selection("List", vec!["First", "Second", "Third"]),

        //lets you select a number with arrow keys
        //arguments: default, step, min, max
        numeric("Numeric", 0.0, 0.5, -5.0, 10.0),

        //buttons when pressed exit the menu
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