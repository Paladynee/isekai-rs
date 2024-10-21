use std::io::Write;

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
};

use crate::component::{TerminalInterface, Universe};

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
    terminterface: &mut TerminalInterface<T>,
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

            if let Err(a) = execute!(terminterface.stdout, Print(c)) {
                return HandlerReturn::Error(a.to_string());
            }
        }

        _ => {}
    }

    HandlerReturn::Continue
}
