use std::fmt::Write as FmtWrite;
use std::sync::RwLockWriteGuard;
use crate::{TerminalMenu, TerminalMenuStruct, TMStatus, TMIKind::*, button, scroll, selected_item_index, get_mutable_instance, TMIKind};
use crate::utils::*;
use crossterm::event;

fn print_item(menu: &mut RwLockWriteGuard<TerminalMenuStruct>, i: usize) {
    //TODO: how many spaces and why?
    print!("{} {}   ", if menu.selected == i { '>' } else { ' ' }, menu.items[i].name);

    for _ in menu.items[i].name.len()..menu.longest_name {
        print!(" ");
    }

    let mut print_buf = String::new();

    match &menu.items[i].kind {
        Label | Button | BackButton | Submenu(_) => {}
        Scroll { values, selected } => write!(print_buf, " {}", values[*selected]).unwrap(),
        List   { values, selected } => {
            for j in 0..values.len() {
                write!(print_buf, "{}{}{}",
                       if j == *selected {'['} else {' '},
                       values[j],
                       if j == *selected {']'} else {' '},
                ).unwrap();
            }
        }
        Numeric { value, .. } => write!(print_buf, " {:.*}", float_printing_precision(*value), value).unwrap(),
    }

    menu.items[i].last_print_len = print_buf.len();
    print!("{}", print_buf);
}
fn print(menu: &mut RwLockWriteGuard<TerminalMenuStruct>) {
    let term_height = term_height();
    match menu.status {
        TMStatus::Normal => {
            for i in 0..menu.items.len() {
                if i != 0 {
                    println!();
                    move_cursor_to_column_0();
                }
                print_item(menu, i);
            }
            move_cursor_to_column_0();
        }
        TMStatus::Altscreen { topmost, .. } => {
            clear_screen();
            move_cursor_to_row(0);
            print!("  ...");
            println!();
            move_cursor_to_column_0();
            for i in 0..term_height - 2 {
                print_item(menu, i + topmost);
                println!();
                move_cursor_to_column_0();
            }
            print!("  ...");
            move_cursor_to_column_0();
        }
        _ => panic!("???")
    }
}
fn unprint(menu: &mut RwLockWriteGuard<TerminalMenuStruct>) {
    match menu.status {
        TMStatus::Normal => {
            move_cursor_up(menu.items.len() - 1);
            clear_from_cursor_down();
        }
        TMStatus::Altscreen { normal_not_printed, .. } => {
            terminal_alt_screen(false);
            if !normal_not_printed {
                move_cursor_up(menu.items.len() - 1);
                clear_from_cursor_down();
            }
        }
        TMStatus::Inactive => panic!()
    }
    menu.status = TMStatus::Inactive;
}
fn change_active_item(menu: &mut RwLockWriteGuard<TerminalMenuStruct>, i: usize) {
    let prev_i = menu.selected;
    menu.selected = i;
    let items_len = menu.items.len();
    match &mut menu.status {
        TMStatus::Normal => {
            save_cursor_pos();
            move_cursor_up(menu.items.len() - prev_i - 1);
            print!(" \u{8}");
            restore_cursor_pos();
            if menu.items.len() <= term_height() {
                move_cursor_up(menu.items.len() - i - 1);
                print!(">");
            }
            restore_cursor_pos();
        }
        TMStatus::Altscreen { topmost, modified, .. } => {
            *modified = true;
            let items_on_screen = term_height() - 2;
            let new_topmost = if i == 0 {
                0
            } else if i == items_len - 1 {
                items_len - items_on_screen
            } else if i <= *topmost {
                i - 1
            } else if i - *topmost > items_on_screen - 2 {
                i - (items_on_screen - 2)
            } else {
                *topmost
            };
            if *topmost == new_topmost {
                move_cursor_to_row(1 + prev_i - *topmost);
                print!(" ");
                move_cursor_to_row(1 + i - *topmost);
                print!(">");
                flush();
            } else {
                *topmost = new_topmost;
                print(menu);
            }
        }
        _ => panic!()
    }
}
fn inc_or_dec_active_item(menu: &mut RwLockWriteGuard<TerminalMenuStruct>, up: bool) {
    let mut i = menu.selected;
    loop {
        if up {
            if i == 0 {
                i = menu.items.len() - 1;
            } else {
                i -= 1;
            }
        } else {
            if i == menu.items.len() - 1 {
                i = 0;
            } else {
                i += 1;
            }
        }
        if let TMIKind::Label = menu.items[i].kind {
            continue;
        }
        break;
    }
    change_active_item(menu, i);
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
fn update_item_value(mut menu: RwLockWriteGuard<TerminalMenuStruct>) {
    save_cursor_pos();
    match menu.status {
        TMStatus::Normal => {
            move_cursor_up(menu.items.len() - menu.selected - 1);
        }
        TMStatus::Altscreen { topmost, normal_not_printed, .. } => {
            move_cursor_to_row(1 + menu.selected - topmost);
            menu.status = TMStatus::Altscreen { topmost, normal_not_printed, modified: true };
        }
        TMStatus::Inactive => panic!()
    }

    //TODO: maybe fix this magic number?
    move_cursor_right(menu.longest_name + 5);

    let mut print_buf = String::new();

    let _selected = menu.selected;
    let menu_selected_item = &mut menu.items[_selected];
    match &mut menu_selected_item.kind {
        Scroll { selected, values} => {
            write!(print_buf, " {}", values[*selected]).unwrap();
        }
        List { selected, values } => {
            for i in 0..values.len() {
                write!(print_buf, "{}{}{}",
                       if i == *selected { '[' } else { ' ' },
                       values[i],
                       if i == *selected { ']' } else { ' ' },
                ).unwrap();
            }
        }
        Numeric { value, .. } => {
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
fn inc_or_dec_item(menu: &TerminalMenu, to_right: bool) {
    let mut menu = menu.write().unwrap();

    let _selected = menu.selected;
    match &mut menu.items[_selected].kind {
        Label | Button | BackButton | Submenu(_) => { return }
        Scroll { selected, values} |
        List   { selected, values } => {
            inc_or_dec_selection_item(selected, values.len(), to_right);
        }
        Numeric { value, step, min, max } => {
            if let Some(step) = step {
                let min = min.unwrap_or(std::f64::MIN);
                let max = max.unwrap_or(std::f64::MAX);
                if to_right {
                    if *value == max {
                        *value = max;
                        return;
                    }
                    *value += *step;
                } else {
                    if *value == min {
                        *value = min;
                        return;
                    }
                    *value -= *step;
                }
            } else {
                return;
            }
        }
    }

    update_item_value(menu);
}

fn handle_enter(menu: &TerminalMenu) {
    let mut menu = menu.write().unwrap();
    let mut update_value = false;
    let is_altscreen = if let TMStatus::Altscreen { .. } = menu.status { true } else { false };

    let _selected = menu.selected;
    if let Numeric { value, step, min, max } = &mut menu.items[_selected].kind {
        if is_altscreen {
            move_cursor_to_bottom();
        }
        terminal_append_line();
        let prefix = number_range_indicator(*step, *min, *max);
        print!("{}", prefix);
        flush();

        cursor_visibility(true);
        loop {
            let mut input = String::new();
            number_input(&mut input);

            if input.is_empty() {
                break;
            }

            if let Ok(v) = input.parse() {
                if value_valid(v, *step, *min, *max) {
                    *value = v;
                    update_value = true;
                    break;
                } else {
                    terminal_append_line();
                    print!("Number not in range! Press enter to cancel.");
                }
            } else {
                terminal_append_line();
                print!(    "Not a valid number! Press enter to cancel. ");
            }

            move_cursor_up(1);
            move_cursor_to_column_0();
            move_cursor_right(prefix.len());
            for _ in 0..input.len() {
                print!(" ");
            }
            move_cursor_left(input.len());
        }
        cursor_visibility(false);

        if is_altscreen {
            print(&mut menu);
        } else {
            move_cursor_to_column_0();
            clear_from_cursor_down();
            move_cursor_up(1);
        }
    }
    else {
        let mut active = false;
        let mut temporary_menu = None;
        let mut item_changed = false;
        match &menu.items[_selected].kind {
            Button | BackButton => {}
            Submenu(submenu) => {
                temporary_menu = Some(submenu.clone());
            }
            Scroll { values, selected } |
            List   { values, selected } => {
                if values.len() != 1 {
                    let mut items = Vec::new();
                    for (i, v) in values.iter().enumerate() {
                        if i == *selected {
                            items.push(scroll(v, vec!["(Selected)"]));
                        } else {
                            items.push(button(v));
                        }
                    }
                    let tm = crate::menu(items);
                    get_mutable_instance(&tm).selected = *selected;
                    temporary_menu = Some(tm);
                    item_changed = true;
                }
            }
            _ => active = true
        }
        menu.active = active;
        menu.temporary_menu = temporary_menu;
        menu.item_changed = item_changed;
    }

    if update_value {
        update_item_value(menu);
    }
}
fn handle_key_event(menu: &TerminalMenu, key_event: event::KeyEvent) {
    use event::KeyCode::*;
    match key_event.code {
        Up | Char('w') => inc_or_dec_active_item(&mut menu.write().unwrap(), true),
        Down | Char('s') => inc_or_dec_active_item(&mut menu.write().unwrap(), false),
        Left  | Char('a') => inc_or_dec_item(&menu, false),
        Right | Char('d') => inc_or_dec_item(&menu, true),
        Enter | Char(' ') => handle_enter(&menu),
        _ => {}
    }
}
fn calc_topmost(menu: &mut RwLockWriteGuard<TerminalMenuStruct>, term_height: usize, topmost: &mut usize) {
    let min_topmost = if menu.selected < (term_height - 4) { 0 } else { menu.selected - (term_height - 4) };
    let max_topmost = (menu.items.len() - 1) - (term_height - 3);
    if *topmost < min_topmost {
        *topmost = min_topmost;
    }
    if *topmost > max_topmost {
        *topmost = max_topmost;
    }
}
fn handle_resize(menu: &mut RwLockWriteGuard<TerminalMenuStruct>, term_height: usize) {
    if menu.items.len() > term_height - 1 {
        match menu.status {
            TMStatus::Inactive => {
                let mut topmost = 0;
                calc_topmost(menu, term_height, &mut topmost);
                menu.status = TMStatus::Altscreen { topmost, normal_not_printed: true, modified: false };
                terminal_alt_screen(true);
                print(menu);
            }
            TMStatus::Normal => {
                let mut topmost = 0;
                calc_topmost(menu, term_height, &mut topmost);
                menu.status = TMStatus::Altscreen { topmost, normal_not_printed: false, modified: false };
                terminal_alt_screen(true);
                print(menu);
            }
            TMStatus::Altscreen { mut topmost, normal_not_printed, modified } => {
                calc_topmost(menu, term_height, &mut topmost);
                menu.status = TMStatus::Altscreen { topmost, normal_not_printed, modified };
                print(menu);
            }
        }
    } else {
        match menu.status {
            TMStatus::Inactive => {
                menu.status = TMStatus::Normal;
                print(menu);
            }
            TMStatus::Normal => {}
            TMStatus::Altscreen { normal_not_printed, modified, .. } => {
                if modified && !normal_not_printed {
                    unprint(menu);
                } else {
                    terminal_alt_screen(false);
                }
                menu.status = TMStatus::Normal;
                if modified || normal_not_printed {
                    print(menu);
                }
            }
        }
    }
}
fn handle_temp_menu(menu: &TerminalMenu, temp_menu: TerminalMenu) -> bool {
    run(temp_menu.clone());
    if menu.read().unwrap().item_changed {
        let menu = &mut menu.write().unwrap();
        let _selected = menu.selected;
        match &mut &mut menu.items[_selected].kind {
            Scroll { selected, .. } |
            List   { selected, .. } => {
                *selected = selected_item_index(&temp_menu);
            }
            _ => panic!("??? code broken dude")
        }
    } else {
        let temp_menu = temp_menu.read().unwrap();
        if let Button = temp_menu.items[temp_menu.selected].kind {
            return false;
        }
    }
    true
}
fn term_mode(on: bool) {
    terminal_raw_mode(on);
    cursor_visibility(!on);
}
pub fn run(menu: TerminalMenu) {

    //set active
    {
        let mut menu = menu.write().unwrap();
        menu.active = true;
        menu.exited = false;
    }

    term_mode(true);

    //print initially
    {
        let mut menu = menu.write().unwrap();
        let mut longest_name = 0;
        for item in &menu.items {
            if item.name.len() > longest_name {
                longest_name = item.name.len();
            }
        }
        menu.longest_name = longest_name;
        handle_resize(&mut menu, term_height());
    }

    while menu.read().unwrap().active {
        if event::poll(SECOND.clone()).unwrap() {
            use event::Event::*;
            match event::read().unwrap() {
                Key(key_event) => handle_key_event(&menu, key_event),
                Resize(_, term_height) => handle_resize(&mut menu.write().unwrap(), term_height as usize),
                Mouse(_) => {}
            }
        }
    }

    let temp_menu = {
        let mut menu = menu.write().unwrap();
        unprint(&mut menu);
        menu.temporary_menu.as_ref().map(|m| m.clone())
    };

    term_mode(false);

    if let Some(temp_menu) = temp_menu {
        if handle_temp_menu(&menu, temp_menu) {
            run(menu);
            return;
        }
    }
    menu.write().unwrap().exited = true;
}