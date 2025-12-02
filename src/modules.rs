use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{Arc, Mutex, mpsc};

pub use crate::hw_config::HardwareConfig;
pub use crate::match_info::MatchInfo;
pub use crate::port_manager::PortManager;
pub use crate::settings_menu::SettingsMenu;
pub use crate::virtuoso_config::VirtuosoConfig;
pub use crate::virtuoso_logger::Logger;

#[derive(Clone, Copy, Debug)]
pub enum CyranoCommand {
    CyranoNext,
    CyranoPrev,
    CyranoBegin,
    CyranoEnd,

    LeftStatus,
    RightStatus,
    LeftVideoAppeal,
    RightVideoAppeal,
    LeftMedical,
    RightMedical,
    LeftReserve,
    RightReserve,
}

#[derive(Clone)]
pub struct VirtuosoModuleContext {
    pub logger: Logger,
    pub config: Arc<Mutex<VirtuosoConfig>>,
    pub hw_config: HardwareConfig,

    match_info_modified_count: Arc<AtomicU32>,
    pub match_info: Arc<Mutex<MatchInfo>>,

    pub settings_menu_shown: Arc<AtomicBool>,
    pub settings_menu: Arc<Mutex<SettingsMenu>>,

    pub cyrano_command_tx: mpsc::Sender<CyranoCommand>,
    pub cyrano_command_rx: Arc<Mutex<Option<mpsc::Receiver<CyranoCommand>>>>,

    pub port_manager: Arc<Mutex<PortManager>>,
}

impl VirtuosoModuleContext {
    pub fn new(
        logger: Logger,
        config: VirtuosoConfig,
        hw_config: HardwareConfig,
        match_info: MatchInfo,
        settings_menu: SettingsMenu,
    ) -> Self {
        let (cyrano_tx, cyrano_rx) = mpsc::channel::<CyranoCommand>();

        Self {
            logger,
            config: Arc::new(Mutex::new(config)),
            hw_config,

            match_info_modified_count: Arc::new(AtomicU32::new(0)),
            match_info: Arc::new(Mutex::new(match_info)),

            settings_menu_shown: Arc::new(AtomicBool::new(false)),
            settings_menu: Arc::new(Mutex::new(settings_menu)),

            cyrano_command_tx: cyrano_tx,
            cyrano_command_rx: Arc::new(Mutex::new(Some(cyrano_rx))),

            port_manager: Arc::new(Mutex::new(PortManager::new())),
        }
    }

    pub fn with_logger(&self, logger: Logger) -> Self {
        let mut res: VirtuosoModuleContext = self.clone();
        res.logger = logger;
        res
    }

    pub fn match_info_data_updated(&mut self) {
        self.match_info_modified_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let raw_ptr: *const AtomicU32 = Arc::as_ptr(&self.match_info_modified_count);
        atomic_wait::wake_all(raw_ptr);
    }

    pub fn get_modified_count(&self) -> u32 {
        self.match_info_modified_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn wait_modified_count_atomic(&self, old_value: u32) {
        atomic_wait::wait(&self.match_info_modified_count, old_value);
    }
}

pub trait VirtuosoModule {
    fn run(self);
}
