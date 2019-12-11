fn main() {
    //it might be a good idea to perform terminal-menu stuff
    //in separate functions from other code
    //so that you can use the following line without much confusion:
    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![

        //lets you select from a list of values with arrow keys
        selection("Selection", vec!["First", "Second", "Third"]),

        //lets you select a number with arrow keys
        //arguments: default, step, min, max
        numeric("Numeric", 0.0, 0.5, -5.0, 10.0),

        //buttons when pressed exit the menu
        button("Option A"),
        button("Option B")

    ]);

    println!("(use arrow keys or wasd)");

    //open the menu
    activate(&menu);

    //wait for the menu to exit
    wait_for_exit(&menu);

    //read values
    {
        //make sure not to hold this variable long
        //if the menu is active
        let read = menu.read().unwrap();
        println!("Selection: {}", read.selection_value("Selection").unwrap());
        println!("Numeric: {}", read.numeric_value("Numeric").unwrap());
        println!("Selected: {}", read.selected_item());
    }
}