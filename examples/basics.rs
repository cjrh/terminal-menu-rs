fn main() {

    //it might be a good idea to perform terminal-menu stuff
    //in separate functions from other code
    //so that you can use the following line without much confusion:
    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![

        //run the example and try these out
        label("(use arrow keys or wasd)"),
        scroll("Selection", vec!["First Option", "Second Option", "Third Option"]),
        list("Do Something", vec!["Yes", "No"]),
        numeric("Numeric", 2.75, Some(0.25), Some(-7.25), Some(11.5)),
        submenu("Submenu", {
            let mut submenu_items = vec![];
            submenu_items.push(numeric("foo", 100.0, None, None, None));
            for i in 0..5 {
                submenu_items.push(label(format!("Section{}", i)));
                for j in 0..5 {
                    submenu_items.push(scroll(format!(" Item{}", j), vec!["A", "B", "C"]));
                }
            }
            submenu_items.push(back_button("back"));
            submenu_items
        }),
        button("Exit")
    ]);

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