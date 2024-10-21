//! ooOOOOoooOOooOOOooOoOOOoOOOOooOOOoOOo (the sound of the universe being created)
//! hello universe
//! i am system
//! you are now being reincarnated... please standby...
#![allow(unused)]
#![allow(clippy::collapsible_match)]

use std::{io, path::Path};

use component::{IsekaiCheatSystem, TerminalInterface, Universe, SAVEFILE_LOCATION};

/// fabric of reality (lol)
mod component;

/// the outside force that is the player
mod event_handler;

/// big bang
fn main() {
    let mut interface = TerminalInterface::new(io::stdout().lock()).unwrap();
    let mut universe =
        IsekaiCheatSystem::load_save_from_file(Path::new(SAVEFILE_LOCATION)).unwrap_or_default();

    interface.start_polling(&mut universe);
}
