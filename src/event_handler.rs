use std::io::Write;

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::component::{Menu, MenuHandler, TerminalInterface, Universe};

/// the user is done with interacting the universe! noo! don't leave yet!
pub enum HandlerReturn<T> {
    Quit,
    Continue,
    Error(T),
}

/// the almighty user has interacted with the universe!
/// what an exciting event! an honor!
pub fn handle_key_event<T: Write>(
    key_event: KeyEvent,
    universe: &mut Universe,
    menu_handler: &mut MenuHandler,
    interface: &mut TerminalInterface<T>,
) -> HandlerReturn<String> {
    if key_event.kind != KeyEventKind::Press {
        return HandlerReturn::Continue;
    }

    #[expect(clippy::single_match)]
    match key_event.code {
        KeyCode::Char(c) => {
            #[expect(clippy::single_match)]
            match c {
                'c' => {
                    if matches!(key_event.modifiers, KeyModifiers::CONTROL) {
                        return HandlerReturn::Quit;
                    }
                }
                _ => {}
            }

            match menu_handler.current_menu {
                Menu::MainMenu => match c {
                    'p' => {
                        menu_handler.current_menu = Menu::Gameplay;
                        if let Err(e) = execute!(interface.stdout, Clear(ClearType::All)) {
                            return HandlerReturn::Error(e.to_string());
                        };
                    }

                    'q' => return HandlerReturn::Quit,

                    's' => {
                        menu_handler.current_menu = Menu::Settings;
                        if let Err(e) = execute!(interface.stdout, Clear(ClearType::All)) {
                            return HandlerReturn::Error(e.to_string());
                        };
                    }

                    'h' => {
                        menu_handler.current_menu = Menu::Help;
                        if let Err(e) = execute!(interface.stdout, Clear(ClearType::All)) {
                            return HandlerReturn::Error(e.to_string());
                        };
                    }

                    _ => {}
                },

                _ => {}
            }

            // if let Err(a) = execute!(interface.stdout, Print(c)) {
            //     return HandlerReturn::Error(a.to_string());
            // }
        }

        _ => {}
    }

    HandlerReturn::Continue
}
