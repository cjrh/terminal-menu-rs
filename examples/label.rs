///
/// Explains the label terminal-menu item kind.
///

fn main() {
    use terminal_menu::{menu, label, button, run};
    run(&menu(vec![

        //labels are not selectable
        //they are useful as titles, separations, sections, etc...
        label("Alicy"),
        label("Box"),
        label("Charlie"),

        //a menu can't contain only labels
        //a selectable item is required
        button("Exit")

    ]));
}