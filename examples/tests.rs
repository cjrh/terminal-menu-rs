fn main() {
    use terminal_menu::*;
    let mut items = Vec::new();

    for i in 0..25 {
        items.push(scroll(&format!("item{}", i), vec!["A", "BB", "CCC"]));
    }
    /*tems.push(numeric("A", 0.0, None, None, None));
    items.push(numeric("A", 0.0, Some(1.0), None, None));
    items.push(numeric("A", 0.0, None, Some(-10.0), None));
    items.push(numeric("A", 0.0, None, None, Some(10.0)));
    items.push(numeric("A", 0.0, Some(1.0), Some(-10.0), None));
    items.push(numeric("A", 0.0, Some(1.0), None, Some(10.0)));
    items.push(numeric("A", 0.0, None, Some(-10.0), Some(10.0)));
    items.push(numeric("A", 0.0, Some(1.0), Some(-10.0), Some(10.0)));*/
    items.push(button("exit"));

    run(&menu(items));
}