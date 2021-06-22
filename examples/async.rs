fn main() {

    use terminal_menu::*;

    //create the menu
    let my_menu = menu(vec![
        scroll("Scroll", vec!["Foo", "Bar"]),
        numeric("Numeric", 0.0, Some(0.5), Some(-10.0), Some(5.0)),
        button("Exit")
    ]);

    //menus can be used asynchronously with the activate() function
    activate(&my_menu);

    //you can do anything you want here
    let mut number = 1;
    for n in 1..10 {
        number *= n;
    }

    //returns when the menu has exited
    wait_for_exit(&my_menu);

    println!("{}", number);
}