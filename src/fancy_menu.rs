use std::io::{stdout, Write as IoWrite, stdin};
use std::fmt::Write as FmtWrite;
use std::time::Duration;
use crate::{TerminalMenu, TerminalMenuStruct, TMIKind::*, button, selected_item_index, get_mutable_instance};
use crate::utils::*;
use crossterm::{execute, event, terminal, cursor};

fn print(menu: &TerminalMenuStruct, longest_name: usize, selected: usize) {
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
            Numeric { value, .. } => print!(" {:.*}", float_printing_precision(*value), value),
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
    } else {
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
fn inc_or_dec_selection_item(selected: &mut usize, values_len: usize, to_right: bool) {
    if to_right {
        if *selected == values_len - 1 {
            *selected = 0;
        } else {
            *selected += 1;
        }
    } else {
        if *selected == 0 {
            *selected = values_len - 1;
        } else {
            *selected -= 1;
        }
    }
}
fn inc_or_dec_item(menu: &TerminalMenu, longest_name: usize, to_right: bool) {
    let mut menu = menu.write().unwrap();

    let _selected = menu.selected;

    if let Button | Submenu(_) = menu.items[_selected].kind {
        return;
    }

    save_cursor_pos();
    move_cursor_up(menu.items.len() - menu.selected - 1);

    //TODO: maybe fix this magic number?
    move_cursor_right(longest_name + 5);

    let mut print_buf = String::new();

    let menu_selected_item = &mut menu.items[_selected];
    match &mut menu_selected_item.kind {
        Scroll { selected, values} => {
            inc_or_dec_selection_item(selected, values.len(), to_right);
            write!(print_buf, " {}", values[*selected]).unwrap();
        }
        List { selected, values } => {
            inc_or_dec_selection_item(selected, values.len(), to_right);
            for i in 0..values.len() {
                write!(print_buf, "{}{}{}",
                       if i == *selected { '[' } else { ' ' },
                       values[i],
                       if i == *selected { ']' } else { ' ' },
                ).unwrap();
            }
        }
        Numeric { value, step, min, max } => {
            if to_right {
                *value += *step;
                if value > max {
                    *value = *max;
                }
            } else {
                *value -= *step;
                if value < min {
                    *value = *min;
                }
            }
            write!(print_buf, " {:.*}", float_printing_precision(*value), value).unwrap();
        }
        _ => panic!("??")
    }

    print!("{}", print_buf);

    let new_print_len = print_buf.len();
    for _ in new_print_len..menu_selected_item.last_print_len {
        print!(" ");
    }
    menu_selected_item.last_print_len = new_print_len;

    restore_cursor_pos();
}
fn handle_enter(menu: &TerminalMenu) {
    let mut menu = menu.write().unwrap();

    let menu_selected = menu.selected;
    if let Numeric { value: _, step, min, max } = &mut menu.items[menu_selected].kind {
        execute!(stdout(), cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();

        print!("\n({:.*}, {:.*} .. {:.*})\nnew value: ",
            float_printing_precision(*min), min,
            float_printing_precision(*min + *step), *min + *step,
            float_printing_precision(*max), max
        );
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();


        //move_cursor_up(2);
        execute!(stdout(),
            cursor::Hide,
            terminal::Clear(terminal::ClearType::FromCursorDown),
            terminal::ScrollDown(3)
        ).unwrap();
        //move_cursor_up(1);
        terminal::enable_raw_mode().unwrap();
    }
    else {
        let mut active = false;
        let mut temporary_menu = None;
        let mut changed_selection_item_index = None;
        match &menu.items[menu_selected].kind {
            Button => {}
            Submenu(submenu) => {
                temporary_menu = Some(submenu.clone());
            }
            Scroll { values, selected } |
            List { values, selected } => {
                let tm = crate::menu(values.iter().map(|v| button(v)).collect());
                get_mutable_instance(&tm).selected = *selected;
                temporary_menu = Some(tm);
                changed_selection_item_index = Some(menu.selected);
            }
            _ => active = true
        }
        menu.active = active;
        menu.temporary_menu = temporary_menu;
        menu.changed_selection_item_index = changed_selection_item_index;
    }
}
fn handle_key_event(menu: &TerminalMenu, longest_name: usize, key_event: event::KeyEvent) {
    use event::KeyCode::*;
    match key_event.code {
        Up | Char('w') => change_active_item(&menu, true),
        Down | Char('s') => change_active_item(&menu, false),
        Left  | Char('a') => inc_or_dec_item(&menu, longest_name, false),
        Right | Char('d') => inc_or_dec_item(&menu, longest_name, true),
        Enter | Char(' ') => handle_enter(&menu),
        _ => {}
    }
}
pub fn handle_temp_menu(menu: &TerminalMenu, temp_menu: TerminalMenu) {
    run(temp_menu.clone());
    let _cii = menu.read().unwrap().changed_selection_item_index;
    if let Some(changed_item_index) = _cii {
        let menu_wr = &mut menu.write().unwrap();
        match &mut menu_wr.items[changed_item_index].kind {
            Scroll { selected, .. } |
            List   { selected, .. } => {
                *selected = selected_item_index(&temp_menu);
            }
            _ => panic!("??? code broken dude")
        }
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
        print(&menu, longest_name, menu.selected);
    }

    terminal::enable_raw_mode().unwrap();

    while menu.read().unwrap().active {
        if event::poll(Duration::from_secs(1)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                handle_key_event(&menu, longest_name, key_event);
            }
        }
    }

    scroll_down(menu.read().unwrap().items.len() - 1);

    execute!(stdout(),
            //cursor::MoveUp(menu.read().unwrap().items.len() as u16 - 1),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Show
    ).unwrap();
    terminal::disable_raw_mode().unwrap();

    let temp_menu = menu.read().unwrap().temporary_menu.as_ref().map(|m| m.clone());

    if let Some(temp_menu) = temp_menu {
        handle_temp_menu(&menu, temp_menu);
        run(menu);
    } else {
        menu.write().unwrap().exited = true;
    }
}