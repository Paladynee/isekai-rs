use std::{
    default,
    fs::File,
    io::{self, BufReader, BufWriter, StdinLock, StdoutLock, Write},
    path::Path,
    time::{Duration, Instant},
};

use bitflags::bitflags;
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::Print,
    terminal::{self, Clear},
};
use serde::{Deserialize, Serialize};
use voxell_utils::time_seeded_rng::TimeSeededXorShift32;

use crate::event_handler::{self, HandlerReturn};

pub const SAVEFILE_LOCATION: &str = "./YOULOSE.undergarments.lol";
pub const SAVE_INTERVAL: Duration = Duration::from_secs(5);

/// damn foes, all you do is watch while your allies are being culled
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttackDetails {
    /// how hard you are hitting the poor thing
    pub damage: f32,
}

/// you vulnerable being, why dont you simply become unattackable??? smh
pub trait Attackable {
    /// attack some puny being. poor thing
    fn attack(&mut self, details: &AttackDetails);
}

/// here's where our healers come in play,
/// they do complex magik of power to heal bazinga
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealDetails {
    /// the bishops are overflowing with mana!!!
    pub power: f32,
}

/// our healers are very stronk and smartsy so they use
/// magik to do funny word and fix wound
pub trait Healable {
    /// heal some puny being. good for you!
    fn heal(&mut self, details: &HealDetails);
}

/// the fuckin main character. wow. so unpredictable.
/// did you know? he's also OP.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Protagonist {
    /// he's also a living thing, bruh
    pub entity: GenericEntityDetails,
    /// legendary hero's doings in this universe
    pub achievements: Achievements,
}

bitflags! {
    /// what mighty deeds you may have completed
    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub struct Achievements: u64 {
        const STARTED_GAME = 1 << 0;
        const SLAIN_ENEMY = 1 << 1;
        const REACHED_LEVEL_1 = 1 << 2;
    }
}

/// the catalogue of interfaces the user can interact with
pub enum Menu {
    /// the initial menu... everything starts here...
    MainMenu,
    /// gameplay interface... wow... user can control so much
    Gameplay,
    /// the control panel, panel of the gods
    Settings,
    /// who dare asks for help???
    Help,
}

/// the assistant of IsekaiCheatSystem that handles the menus for user
pub struct MenuHandler {
    /// what the user can do right now
    pub current_menu: Menu,
}

impl MenuHandler {
    pub const fn new(menu: Menu) -> Self {
        Self { current_menu: menu }
    }
}

impl Default for Protagonist {
    /// so generic... wasn't he supposed to be OP?
    fn default() -> Self {
        Self {
            entity: GenericEntityDetails {
                power: 10.0,
                /// beta male
                tier: EntityTier::H,
                level: 1,
                exp: 0.0,
                hp: 100.0,
                mana: 0.0,
            },
            achievements: Achievements::empty(),
        }
    }
}

/// some common things that all living things share... wow... so creative
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenericEntityDetails {
    /// how stronk it is
    pub power: f32,
    /// how mighty it is
    pub tier: EntityTier,
    /// how wise it is
    pub level: u32,
    /// how experienced it is with its current wisdom
    pub exp: f32,
    /// how healthy it is
    pub hp: f32,
    /// how manaful it is
    pub mana: f32,
}

/// one of the rules that govern this world.
/// higher your tier = stronger you are.
/// lower tier organisms are much weaker against you.
#[derive(
    Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, Default,
)]
pub enum EntityTier {
    /// slime (exclusively)
    #[default]
    H,
    /// average small animal
    G,
    /// average adult male
    F,
    /// a man who can singlehandedly destroy a gang
    E,
    /// army level danger
    D,
    /// threat to our country
    C,
    /// continental killer
    B,
    /// planet slaughterer
    A,
    /// can kill star system
    S,
    /// he destroy galaxy
    SS,
    /// a being that destroy universe (not rely)
    FortniteBattleGod,
}

/// the thing that contains everything else, namely, the universe (duh)
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Universe {
    /// you, because u'r literally the main character bro, damn.
    pub you: Protagonist,
}

impl Universe {
    /// the universe should have a time dimension, shouldn't it?
    pub fn pass_time(&mut self, time: Duration) {}
}

/// there's something bigger than the universe!?!?!? what the fuck?
pub struct Multiverse<T>
where
    T: Write,
{
    /// puny universe, lol
    pub universe: Universe,
    /// blessed be the RNGesus, for he decides our fate
    pub rng: TimeSeededXorShift32,
    /// the user also belongs here??? what?
    pub interface: TerminalInterface<T>,
    /// a... menu? what is that?
    pub menu_handler: MenuHandler,
}

impl<T> Multiverse<T>
where
    T: Write,
{
    /// operating on this level of highness is making me dizzy..
    /// so... much... power...
    pub const fn new(
        universe: Universe,
        rng: TimeSeededXorShift32,
        interface: TerminalInterface<T>,
        menu_handler: MenuHandler,
    ) -> Self {
        Self {
            universe,
            rng,
            interface,
            menu_handler,
        }
    }
}

/// WOW! isekai cheat system in front of my very eyes! what the fuck?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IsekaiCheatSystem;

impl IsekaiCheatSystem {
    /// wow, system really actually does something!
    /// she pulls data from some omniverse and remakes the universe!
    /// how skillful~! (*>﹏<*)′
    pub fn load_save_from_file(location: &Path) -> io::Result<Universe> {
        let mut file = File::open(location)?;
        let mut reader = BufReader::new(&file);
        let res: Universe = serde_json::from_reader(reader)?;
        Ok(res)
    }

    /// wow, system is capable of doing that??!?!?!
    /// the entire universe at the palm of her hands...
    /// this much power should be restricted...
    pub fn write_save_to_file(location: &Path, uni: &Universe) -> io::Result<()> {
        let mut file = File::create(location)?;
        let mut writer = BufWriter::new(&file);
        serde_json::to_writer(&mut writer, uni)?;
        writer.flush()?;
        Ok(())
    }
}

/// this is made so you can control the game, lol
pub struct TerminalInterface<T>
where
    T: Write,
{
    /// uhm ohm... the... um... user interface thing
    pub stdout: T,
}

impl<T> TerminalInterface<T>
where
    T: Write,
{
    /// o holy terminal, please bless us with your presence
    pub fn new(mut stdout: T) -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(&mut stdout, terminal::EnterAlternateScreen)?;

        Ok(Self { stdout })
    }

    /// o terminal, start doing cool backflips for some reason
    pub fn start_polling(
        &mut self,
        universe: &mut Universe,
        rng: &mut TimeSeededXorShift32,
        menu_handler: &mut MenuHandler,
    ) -> io::Result<()> {
        let mut last_update = Instant::now();
        let mut last_save = Instant::now();
        let poll_duration = Duration::from_secs(1) / 60;

        match menu_handler.current_menu {
            Menu::MainMenu => {
                self.show_main_menu()?;
            }
            _ => {}
            _ => {}
        }

        loop {
            let res = event::poll(poll_duration)?;
            let elapsed = last_update.elapsed();

            if last_save.elapsed() > SAVE_INTERVAL {
                IsekaiCheatSystem::write_save_to_file(Path::new(SAVEFILE_LOCATION), universe)?;
                last_save = Instant::now();
            }

            let event = if res {
                event::read()?
            } else {
                universe.pass_time(elapsed);
                continue;
            };

            match event {
                Event::Key(a) => {
                    let res = event_handler::handle_key_event(a, universe, menu_handler, self);
                    match res {
                        HandlerReturn::Quit => break,
                        HandlerReturn::Continue => {}
                        HandlerReturn::Error(e) => {
                            execute!(self.stdout, Print(e))?;
                        }
                    }
                }
                Event::Resize(nx, ny) => match menu_handler.current_menu {
                    Menu::MainMenu => {
                        self.show_main_menu()?;
                    }
                    _ => {}
                    _ => {}
                },
                _ => {}
            }

            last_update = Instant::now();
        }

        Ok(())
    }

    pub fn show_main_menu(&mut self) -> io::Result<()> {
        let (w, h) = terminal::size()?;
        let title_text = "ISEKAI CHEAT SYSTEM";
        let controls = "P: Play Game | Q: Quit | S: Settings | H: Help";

        queue!(
            self.stdout,
            Clear(terminal::ClearType::All),
            MoveTo(w / 2 - (title_text.len() as u16 / 2), h / 2),
            Print(title_text),
            MoveTo(w / 2 - (controls.len() as u16 / 2), h / 2 + 1),
            Print(controls),
            Hide,
        )?;

        self.stdout.flush()?;
        Ok(())
    }
}

impl<T> Drop for TerminalInterface<T>
where
    T: Write,
{
    /// o evil terminal, please leave us alone
    fn drop(&mut self) {
        /// we shant double panic if we panicking already, so no unwrap
        let _ = execute!(self.stdout, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}
