use terminal_menu::{
    display,
    TerminalMenuItem,
    MultiSelectTerminalMenuItem,
    NumericTerminalMenuItem
};

fn main() {

    let mut menu1: Vec<(String, Box<dyn TerminalMenuItem>)> = vec![
        ("Multi".to_owned(), Box::new((MultiSelectTerminalMenuItem::new(vec![
            "First".to_owned(),
            "Second".to_owned(),
            "Third".to_owned(),
        ])))),
        ("Numeric".to_owned(), Box::new(NumericTerminalMenuItem::new(
            0.0,
            -5.0,
            0.5,
            1.0))),
        ("Submenu".to_owned(), Box::new(())),
        ("Exit".to_owned(), Box::new(()))
    ];
    let mut menu2: Vec<(String, Box<dyn TerminalMenuItem>)> = vec![
        ("AnotherMulti".to_owned(), Box::new(MultiSelectTerminalMenuItem::new(vec![
            "First".to_owned(),
            "Second".to_owned(),
            "Third".to_owned(),
        ]))),
        ("AnotherNumeric".to_owned(), Box::new(NumericTerminalMenuItem::new(
            0.0,
            -5.0,
            0.5,
            1.0))),
        ("Exit".to_owned(), Box::new(()))
    ];

    loop {
        let result = display(&mut menu1, |_,_,_| {} ,true);
        if result == 2 {
            display(&mut menu2, |_,_,_| {}, true);
        }
        if result == 3 {
            break;
        }
    }
}