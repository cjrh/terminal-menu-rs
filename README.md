# terminal-menu-rs
Display simple menus on the terminal  
[Examples](https://gitlab.com/xamn/terminal-menu-rs/tree/master/examples)
```
> Selection       Second Option
  Do Something   [Yes] No
  Numeric         5.25
  Submenu    
  Exit     
```

### v1.9.7

- scrolling now possible when items do not fit on the screen
- updated to crossterm 0.17.3
- fixed bug when menu would break when resizing terminal in Windows
- reformatted code
- mutating the menu (when not active) can now be done safely with get_mutable_instance()
- numeric values can now be typed in
- other small features
- thanks to Vectole from gitlab for ideas on how to improve the crate

[kriikkula.com/contact](https://kriikkula.com/contact)