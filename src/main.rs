//! ooOOOOoooOOooOOOooOoOOOoOOOOooOOOoOOo (the sound of the universe being created)
//! hello universe
//! i am system
//! you are now being reincarnated... please standby...
#![allow(unused)]
#![allow(clippy::collapsible_match)]

use std::{io, path::Path};

use component::{
    IsekaiCheatSystem, Menu, MenuHandler, Multiverse, TerminalInterface, Universe,
    SAVEFILE_LOCATION,
};
use voxell_utils::time_seeded_rng::TimeSeededXorShift32;

/// fabric of reality (lol)
mod component;

/// the outside force that is the player
mod event_handler;

/// big bang
fn main() {
    let mut multiverse = Multiverse::new(
        IsekaiCheatSystem::load_save_from_file(Path::new(SAVEFILE_LOCATION)).unwrap_or_default(),
        TimeSeededXorShift32::new().unwrap(),
        TerminalInterface::new(io::stdout()).unwrap(),
        MenuHandler::new(Menu::MainMenu),
    );

    multiverse
        .interface
        .start_polling(
            &mut multiverse.universe,
            &mut multiverse.rng,
            &mut multiverse.menu_handler,
        )
        .unwrap();
}
