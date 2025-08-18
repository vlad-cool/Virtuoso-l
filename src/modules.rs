use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{Arc, Mutex, mpsc};

pub use crate::hw_config::HardwareConfig;
pub use crate::match_info::MatchInfo;
pub use crate::settings_menu::SettingsMenu;
pub use crate::virtuoso_config::VirtuosoConfig;
pub use crate::virtuoso_logger::Logger;

#[derive(Clone, Copy, Debug)]
pub enum CyranoCommand {
    CyranoNext,
    CyranoPrev,
    CyranoBegin,
    CyranoEnd,
}

#[derive(Clone)]
pub struct VirtuosoModuleContext {
    pub logger: Logger,
    pub config: Arc<Mutex<VirtuosoConfig>>,
    pub hw_config: HardwareConfig,

    pub match_info_modified_count: Arc<AtomicU32>,
    pub match_info: Arc<Mutex<MatchInfo>>,

    pub settings_menu_shown: Arc<AtomicBool>,
    pub settings_menu: Arc<Mutex<SettingsMenu>>,

    pub cyrano_command_tx: mpsc::Sender<CyranoCommand>,
    pub cyrano_command_rx: Arc::<Mutex<Option<mpsc::Receiver<CyranoCommand>>>>,
}

impl VirtuosoModuleContext {
    pub fn new(
        logger: Logger,
        config: VirtuosoConfig,
        hw_config: HardwareConfig,
        match_info: MatchInfo,
        settings_menu: SettingsMenu,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<CyranoCommand>();

        Self {
            logger,
            config: Arc::new(Mutex::new(config)),
            hw_config,

            match_info_modified_count: Arc::new(AtomicU32::new(0)),
            match_info: Arc::new(Mutex::new(match_info)),

            settings_menu_shown: Arc::new(AtomicBool::new(false)),
            settings_menu: Arc::new(Mutex::new(settings_menu)),

            cyrano_command_tx: tx,
            cyrano_command_rx: Arc::new(Mutex::new(Some(rx))),
        }
    }

    pub fn with_logger(&self, logger: Logger) -> Self {
        let mut res: VirtuosoModuleContext = self.clone();
        res.logger = logger;
        res
    }
}

pub trait VirtuosoModule {
    fn run(self);
}
