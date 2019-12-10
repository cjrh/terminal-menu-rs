use std::thread::sleep;
use std::time::Duration;

fn main() {
    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![
        selection("Selection", vec!["First", "Second", "Third"]),
        numeric("Numeric", 0.0, 0.5, -5.0, 10.0),
        button("Exit")
    ]);

    //open the menu
    activate(&menu);


    //wait for the menu to exit
    while look(&menu).is_active() {
        sleep(Duration::from_millis(10));
    }

    //read values

    //make sure not to hold a look variable while menu is active too long!
    //this will cause the menu to get stuck!
    let look = look(&menu);

    println!("Selection: {}", look.selection_value("Selection"));
    println!("Numeric: {}", look.numeric_value("Numeric"));
    println!("Selected: {}", look.selected_item());
}