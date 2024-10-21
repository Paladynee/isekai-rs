use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::component::Universe;

/// the user is done with interacting the universe! noo! don't leave yet!
pub enum HandlerReturn<T> {
    Quit,
    Continue,
    Error(T),
}

/// the almighty user has interacted with the universe!
/// what an exciting event! an honor!
pub fn handle_key_event(key_event: KeyEvent, universe: &mut Universe) -> HandlerReturn<String> {
    #[expect(clippy::single_match)]
    match key_event.code {
        KeyCode::Char(c) =>
        {
            #[expect(clippy::single_match)]
            match c {
                'c' =>
                {
                    #[expect(clippy::single_match)]
                    match key_event.modifiers {
                        KeyModifiers::CONTROL => {
                            return HandlerReturn::Quit;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        _ => {}
    }

    HandlerReturn::Continue
}
