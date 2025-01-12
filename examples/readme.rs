fn main() {
    use terminal_menu::{run, menu, label, scroll, list, string, numeric, submenu, back_button};
    let menu = menu(vec![
        label("--------------"),
        label("MY lovely menu!"),
        label("usage: tinker around"),
        label("---------------"),
        scroll("Selection", vec!["First Option", "Second Option", "Third Option"]),
        list("Do Something", vec!["Yes", "No"]),
        string("Your Name", "Samuel", false),
        numeric("Numeric", 5.25, None, None, None),
        submenu("Submenu", vec![back_button("Back")]),
        back_button("Exit"),
    ]);
    run(&menu);
}