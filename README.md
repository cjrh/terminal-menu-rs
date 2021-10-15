# terminal-menu-rs
Display simple menus on the terminal.
[Examples](https://gitlab.com/xamn/terminal-menu-rs/tree/master/examples)
```
> Selection       Second Option
  Do Something   [Yes] No
  Your Name       Samuel          
  Numeric         5.25
  Submenu    
  Exit     
```

### v2.0.1
- fixed bug where terminal would exit alternate screen on submenu exit

### v2.0.0

- updated to crossterm 0.20.0
- complete backend rewrite
- color [yay :)]
- removed clumsy helper functions
- added string terminal-menu items
- bugfixes

#### Migrating from v1.9.7
- `get_mutable_instace` was renamed to `mut_menu`
- Rewrite helper functions  
from: `selection_value(&menu, "foo")`  
to: `mut_menu(&menu).selection_value("foo")`
- See the examples! Lot's of good stuff there!

[kriikkula.com](https://kriikkula.com/)
