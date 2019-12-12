fn main() {
    use terminal_menu::*;

    let main_menu = menu(vec![
        button("First Menu"),
        button("Second Menu"),
        button("Exit")
    ]);
    let first_menu = menu(vec![
        list_selection("Selection", vec!["A", "B", "C"]),
        button("Back")
    ]);
    let second_menu = menu(vec![
        numeric("Numeric", -4.0, 2.0, -10.0, 12.0),
        button("Back")
    ]);

    loop {

        //run activates and waits for exit
        run(&main_menu);

        //terminal_menu uses RwLock, as seen here
        //i suggest you read the RwLock documentation

        match main_menu.read().unwrap().selected_item() {
            "First Menu"  => run(&first_menu),
            "Second Menu" => run(&second_menu),
            _ => break
        }
    }

    println!("Selection: {}", selection_value(&first_menu, "Selection").unwrap());
    println!("Numeric: {}", numeric_value(&second_menu, "Numeric").unwrap());
}