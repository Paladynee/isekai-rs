pub mod entity;
pub mod event_handler;
pub mod interface;
pub mod menu;
pub mod multiverse;
pub mod protagonist;
pub mod system;
pub mod universe;

use std::time::Duration;

pub const SAVEFILE_LOCATION: &str = "./YOULOSE.undergarments.lol";
pub const SAVE_INTERVAL: Duration = Duration::from_secs(5);
