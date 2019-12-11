fn main() {
    use terminal_menu::*;

    let first_menu = menu(vec![
        button("Some Menu"),
        button("Other Menu"),
        button("Exit")
    ]);
    let some_menu = menu(vec![
        selection("Option", vec!["A", "B"]),
        button("Back")
    ]);
    let other_menu = menu(vec![
        numeric("Option", 5.0, 0.25, 0.0, 10.0),
        button("Back")
    ]);

    loop {
        activate(&first_menu);
        wait_for_exit(&first_menu);

        match first_menu.read().unwrap().selected_item() {
            "Some Menu" => {
                activate(&some_menu);
                wait_for_exit(&some_menu);
            },
            "Other Menu" => {
                activate(&other_menu);
                wait_for_exit(&other_menu);
            }
            _ => break
        }
    }

}