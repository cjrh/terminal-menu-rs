///
/// Shows hoc terminal-menu items can be colored.
///

fn main() {

    use terminal_menu::*;
    use crossterm::style::Color;

    let menu = menu(vec![
        //colors!
        label("I'm Green!").colorize(Color::Green),
        button("I'm Blue! (da ba dee)").colorize(Color::Blue),
        list("I'm Red", vec!["foo", "bar", "baz"]).colorize(Color::Red),
        string("I'm plain white", "Me too", false)
    ]);

    run(&menu);
}