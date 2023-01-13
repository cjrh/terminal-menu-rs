///
/// Use menus asynchronously.
///

fn main() {
    use terminal_menu::{menu, label, button, activate, wait_for_exit};
    let menu = menu(vec![
        label("do work when menu open!"),
        button("get me out of here!")
    ]);

    // like run but doesn't block
    activate(&menu);

    // do stuff
    let mut num: usize = 1;
    for i in 2..10 {
        num *= i;
    }

    wait_for_exit(&menu);
    println!("{}", num);
}