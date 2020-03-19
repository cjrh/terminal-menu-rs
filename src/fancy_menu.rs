use std::io::{stdout, Write as IoWrite};
use std::fmt::Write as FmtWrite;
use std::time::Duration;
use crate::{TerminalMenu, TerminalMenuStruct, TMIKind::{self, *}};
use crate::utils::*;
use crossterm::{execute, event, terminal, cursor};

fn print_menu(menu: &TerminalMenuStruct, longest_name: usize, selected: usize) {
    for i in 0..menu.items.len() {
        if i != 0 {
            println!();
        }
        if i == menu.items.len() - 1 {
            save_cursor_pos();
        }

        //TODO: how many spaces and why?
        print!("{} {}   ", if i == selected { '>' } else { ' ' }, menu.items[i].name);

        for _ in menu.items[i].name.len()..longest_name {
            print!(" ");
        }
        match &menu.items[i].kind {
            Button | Submenu(_) => {}
            Scroll { values, selected } => print!(" {}", values[*selected]),
            List   { values, selected } => {
                for j in 0..values.len() {
                    print!("{}{}{}",
                           if j == *selected {'['} else {' '},
                           values[j],
                           if j == *selected {']'} else {' '},
                    );
                }
            }
            Numeric { value, .. } => print!(" {}", value),
        }
    }
    restore_cursor_pos();
    stdout().lock().flush().unwrap();
}
fn change_active_item(menu: &TerminalMenu, up: bool) {
    let mut menu = menu.write().unwrap();

    save_cursor_pos();
    move_cursor_up(menu.items.len() - menu.selected - 1);
    print!(" ");

    if up {
        if menu.selected == 0 {
            menu.selected = menu.items.len() - 1;
            move_cursor_down(menu.items.len() - 1);
        } else {
            menu.selected -= 1;
            move_cursor_up(1);
        }
    }
    else {
        if menu.selected == menu.items.len() - 1 {
            menu.selected = 0;
            move_cursor_up(menu.items.len() - 1);
        } else {
            menu.selected += 1;
            move_cursor_down(1);
        }
    }

    print!("\u{8}>");

    restore_cursor_pos();
}
#[derive(Eq, PartialEq, Copy, Clone)]
enum Action {
    Left,
    Right,
    Enter
}
fn change_selection_on_selection_item(selected: &mut usize, values: &mut Vec<String>, action: Action) {
    if action == Action::Left {
        if *selected == 0 {
            *selected = values.len() - 1;
        } else {
            *selected -= 1;
        }
    }
    else {
        if *selected == values.len() - 1 {
            *selected = 0;
        } else {
            *selected += 1;
        }
    }
}
fn use_menu_item(menu: &TerminalMenu, longest_name: usize, action: Action) {
    let mut menu = menu.write().unwrap();

    save_cursor_pos();
    move_cursor_up(menu.items.len() - menu.selected - 1);

    //TODO: maybe fix this magic number?
    move_cursor_right(longest_name + 5);

    let _selected = menu.selected;
    let menu_selected_item = &mut menu.items[_selected];
    if let Button | Submenu(_) = &menu_selected_item.kind {
        if action == Action::Enter {
            menu.active = false;
        }
    }
    else {
        let mut print_buf = String::new();
        match &mut menu_selected_item.kind {
            Scroll { selected, values} => {
                change_selection_on_selection_item(selected, values, action);
                write!(print_buf, " {}", values[*selected]).unwrap();
            }
            List { selected, values } => {
                change_selection_on_selection_item(selected, values, action);
                for i in 0..values.len() {
                    write!(print_buf, "{}{}{}",
                           if i == *selected { '[' } else { ' ' },
                           values[i],
                           if i == *selected { ']' } else { ' ' },
                    ).unwrap();
                }
            }
            Numeric { value, step, min, max } => {
                if action == Action::Right {
                    *value += *step;
                    if value > max {
                        *value = *max;
                    }
                } else if action == Action::Left {
                    *value -= *step;
                    if value < min {
                        *value = *min;
                    }
                }
                write!(print_buf, " {}", *value).unwrap();
            }
            _ => panic!("update match above")
        }

        print!("{}", print_buf);

        let new_print_len = print_buf.len();
        for _ in new_print_len..menu_selected_item.last_print_len {
            print!(" ");
        }
        menu_selected_item.last_print_len = new_print_len;
    }

    restore_cursor_pos();
}
fn handle_key_event(menu: &TerminalMenu, longest_name: usize, key_event: event::KeyEvent) {
    use event::KeyCode::*;
    match key_event.code {
        Up | Char('w') => change_active_item(&menu, true),
        Down | Char('s') => change_active_item(&menu, false),
        Left  | Char('a') => use_menu_item(&menu, longest_name, Action::Left),
        Right | Char('d') => use_menu_item(&menu, longest_name, Action::Right),
        Enter | Char(' ') => use_menu_item(&menu, longest_name, Action::Enter),
        _ => {}
    }
}
pub fn run(menu: TerminalMenu) {

    //set active
    {
        let mut menu = menu.write().unwrap();
        menu.active = true;
        menu.exited = false;
    }

    execute!(stdout(), cursor::Hide).unwrap();

    //print initially
    let mut longest_name = 0;
    {
        let menu = menu.read().unwrap();
        for item in &menu.items {
            if item.name.len() > longest_name {
                longest_name = item.name.len();
            }
        }
        print_menu(&menu, longest_name, menu.selected);
    }

    terminal::enable_raw_mode().unwrap();

    while menu.read().unwrap().active {
        if event::poll(Duration::from_secs(1)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                handle_key_event(&menu, longest_name, key_event);
            }
        }
    }

    execute!(stdout(),
            cursor::MoveUp(menu.read().unwrap().items.len() as u16 - 1),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Show
    ).unwrap();
    terminal::disable_raw_mode().unwrap();

    let mut runagain = false;
    {
        let menu_rd = menu.read().unwrap();
        if let Submenu(submenu) = &menu_rd.items[menu_rd.selected].kind {
            submenu.write().unwrap().selected = 0;
            run(submenu.clone());
            runagain = true;
        }
    }

    if runagain {
        run(menu);
    }
    else {
        menu.write().unwrap().exited = true;
    }
}