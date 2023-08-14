fn main() {
    use terminal_menu::{
        back_button, label, list, menu, numeric, password, run, scroll, string, submenu,
    };
    let menu = menu(vec![
        label("--------------"),
        label("MY lovely menu!"),
        label("usage: tinker around"),
        label("---------------"),
        scroll(
            "Selection",
            vec!["First Option", "Second Option", "Third Option"],
        ),
        list("Do Something", vec!["Yes", "No"]),
        string("Your Name", "Samuel", false),
        password("Some Password", false),
        numeric("Numeric", 5.25, None, None, None),
        submenu("Submenu", vec![back_button("Back")]),
        back_button("Exit"),
    ]);
    run(&menu);
}
