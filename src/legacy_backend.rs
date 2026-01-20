#[cfg(feature = "legacy_backend_full")]
use gpio_cdev;
#[cfg(feature = "legacy_backend_full")]
use gpio_cdev::Line;
use serialport::SerialPort;

use std::io::Read;
use std::sync::mpsc::RecvError;
use std::sync::{MutexGuard, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use crate::match_info::{self, Weapon};
use crate::modules::{self, CyranoCommand, MatchInfo, VirtuosoModuleContext};
use crate::virtuoso_config::VirtuosoConfig;
use crate::virtuoso_logger::Logger;
#[cfg(feature = "legacy_backend")]
use crate::virtuoso_logger::LoggerUnwrap;

const AUTO_STATUS_WAIT_THRESHOLD: std::time::Duration = std::time::Duration::from_millis(200);
const AUTO_STATUS_ON: u32 = 196;
const AUTO_STATUS_OFF: u32 = 17;

pub struct LegacyBackend {
    context: VirtuosoModuleContext,

    #[cfg(feature = "legacy_backend_full")]
    weapon_select_btn_pressed: bool,
    #[cfg(feature = "legacy_backend_full")]
    rc5_address: u32,
    auto_status_controller: AutoStatusController,

    #[cfg(feature = "legacy_backend_full")]
    rc5_tx: Option<mpsc::SyncSender<IrFrame>>,

    reset_passive: bool,
    last_second: bool,

    epee_5_counter: u32,
    prev_got_time: Duration,

    prev_uart_msg: Option<UartData>,
}

impl modules::VirtuosoModule for LegacyBackend {
    fn run(mut self) {
        let (tx, rx) = mpsc::sync_channel::<InputData>(8);

        let tx_clone: mpsc::SyncSender<InputData> = tx.clone();

        if let Ok(mut port) = self.context.port_manager.lock().unwrap().get_port(
            self.context.hw_config.legacy_backend.uart_port.clone(),
            38400,
        ) {
            port.set_timeout(Duration::from_secs(60))
                .log_err(&self.context.logger);

            thread::spawn(move || {
                uart_handler(tx_clone, port);
            });
        } else {
            self.context
                .logger
                .critical_error("Failed to get port".to_string());
        }

        #[cfg(feature = "legacy_backend_full")]
        {
            let tx_clone: mpsc::SyncSender<InputData> = tx.clone();
            let logger_clone: Logger = self.context.logger.clone();

            let gpio_line_weapon_0: Line = self
                .context
                .hw_config
                .legacy_backend
                .weapon_0_pin
                .to_line()
                .unwrap_with_logger(&self.context.logger);
            let gpio_line_weapon_1: Line = self
                .context
                .hw_config
                .legacy_backend
                .weapon_1_pin
                .to_line()
                .unwrap_with_logger(&self.context.logger);
            let gpio_line_weapon_btn: Line = self
                .context
                .hw_config
                .legacy_backend
                .weapon_btn_pin
                .to_line()
                .unwrap_with_logger(&self.context.logger);

            thread::spawn(move || {
                pins_handler(
                    tx_clone,
                    logger_clone,
                    gpio_line_weapon_0,
                    gpio_line_weapon_1,
                    gpio_line_weapon_btn,
                );
            });
        }

        #[cfg(feature = "legacy_backend_full")]
        {
            let tx: mpsc::SyncSender<InputData> = tx.clone();
            let logger_clone: Logger = self.context.logger.clone();
            let ir_line_rx: Line = self
                .context
                .hw_config
                .legacy_backend
                .ir_pin_rx
                .to_line()
                .unwrap_with_logger(&self.context.logger);

            let ir_line_tx: Line = self
                .context
                .hw_config
                .legacy_backend
                .ir_pin_tx
                .to_line()
                .unwrap_with_logger(&self.context.logger);

            // let line_clone: Line = ir_line.clone();
            // let pause_ir_receiver_1: Arc<AtomicBool> = pause_ir_receiver.clone();
            // let pause_ir_receiver_2: Arc<AtomicBool> = pause_ir_receiver_1.clone();
            thread::spawn(move || {
                rc5_receiever(tx, logger_clone, ir_line_rx);
            });

            let (ir_transmitter_tx, ir_transmitter_rx) = mpsc::sync_channel::<IrFrame>(8);

            self.rc5_tx = Some(ir_transmitter_tx);

            let logger_clone: Logger = self.context.logger.clone();
            thread::spawn(move || {
                rc5_transmitter(ir_transmitter_rx, logger_clone, ir_line_tx);
            });
        }

        loop {
            match rx.recv() {
                Err(RecvError) => {}
                Ok(msg) => {
                    match msg {
                        InputData::UartData(msg) => {
                            self.apply_uart_data(msg);
                        }
                        #[cfg(feature = "legacy_backend_full")]
                        InputData::PinsData(msg) => {
                            self.apply_pins_data(msg);
                        }
                        #[cfg(feature = "legacy_backend_full")]
                        InputData::IrCommand(msg) => {
                            self.apply_ir_data(msg);
                        }
                    };
                    self.set_auto_statuses();
                }
            }
        }
    }
}

impl LegacyBackend {
    pub fn new(context: VirtuosoModuleContext) -> Self {
        #[cfg(feature = "legacy_backend_full")]
        let rc5_address: u32 = context.config.lock().unwrap().legacy_backend.rc5_address;

        {
            let mut match_info_data: MutexGuard<'_, modules::MatchInfo> =
                context.match_info.lock().unwrap();
            let config: &crate::virtuoso_config::LegacyBackendConfig =
                &context.config.lock().unwrap().legacy_backend;

            match_info_data.auto_score_on = config.auto_score_on;
            match_info_data.auto_timer_on = config.auto_timer_on;
        }

        Self {
            context,

            #[cfg(feature = "legacy_backend_full")]
            weapon_select_btn_pressed: false,
            #[cfg(feature = "legacy_backend_full")]
            rc5_address,
            auto_status_controller: AutoStatusController::new(),

            #[cfg(feature = "legacy_backend_full")]
            rc5_tx: None,

            reset_passive: false,
            last_second: false,

            epee_5_counter: 0,
            prev_got_time: Duration::from_secs(3 * 60),

            prev_uart_msg: None,
        }
    }

    fn reset_passive_timer(match_info: &mut MatchInfo) {
        match_info.timer_controller.reset_passive_timer();
        match_info
            .timer_controller
            .set_passive_timer_active(match_info.weapon != Weapon::Sabre);
    }

    #[cfg(feature = "legacy_backend_full")]
    fn mark_left_fencer(&mut self, msg: UartData) {
        if !msg.on_timer {
            if !(msg.yellow_card_left
                || msg.red_card_left
                || msg.yellow_card_right
                || msg.red_card_right)
            {
                if let Some(tx) = &self.rc5_tx {
                    tx.send(IrFrame {
                        new: true,
                        address: self.context.hw_config.legacy_backend.rc5_output_addr,
                        command: IrCommands::LeftPenaltyCard,
                    })
                    .log_err(&self.context.logger);
                }
            }
        }
    }

    fn set_time(
        match_info: &mut MatchInfo,
        msg: UartData,
        prev_got_time: &mut Duration,
        last_second: &mut bool,
    ) {
        let timer_m: u32 = if msg.period == 0b1100 { 4 } else { msg.minutes };
        let timer_d: u32 = msg.dec_seconds;
        let timer_s: u32 = msg.seconds;

        *last_second = match (timer_m, timer_d, timer_s) {
            (0, 0, 0) => false,
            (0, 9, _) => true,
            (_, 0, 0) => false,
            _ => *last_second,
        };

        let new_time: Duration = if *last_second {
            Duration::from_millis((timer_d * 100 + timer_s * 10) as u64)
        } else {
            Duration::from_secs((timer_m * 60 + timer_d * 10 + timer_s) as u64)
        };

        if new_time != *prev_got_time {
            match (match_info.timer_controller.is_timer_running(), msg.on_timer) {
                (true, true) => match_info.timer_controller.sync(new_time, false),
                (true, false) => {}
                (false, true) => {}
                (false, false) => match_info.timer_controller.sync(new_time, true),
            }
        }
        // if !(msg.on_timer & )
        // if new_time != *prev_got_time {
        //
        // }
        match_info.timer_controller.start_stop(msg.on_timer);

        *prev_got_time = new_time;
    }

    fn set_score(match_info: &mut MatchInfo, msg: UartData) {
        if match_info.timer_controller.is_timer_running() {
            if msg.score_left == match_info.left_fencer.score + 1 && !msg.on_timer {
                match_info.left_fencer.score_auto_updated = Some(Instant::now());
            }
            if msg.score_right == match_info.right_fencer.score + 1 && !msg.on_timer {
                match_info.right_fencer.score_auto_updated = Some(Instant::now());
            }
        }

        match_info.left_fencer.score = msg.score_left;
        match_info.right_fencer.score = msg.score_right;
    }

    fn set_priority(match_info: &mut MatchInfo, msg: UartData) {
        match_info.priority = match msg.period {
            0b1110 => match_info::Priority::Left,
            0b1111 => match_info::Priority::Right,
            0b1011 => match_info::Priority::None,
            _ => match match_info.priority {
                match_info::Priority::Right => match_info::Priority::Right,
                match_info::Priority::Left => match_info::Priority::Left,
                match_info::Priority::None => match_info::Priority::None,
            },
        };
    }

    fn apply_uart_data(&mut self, msg: UartData) {
        #[cfg(feature = "legacy_backend_full")]
        self.mark_left_fencer(msg);

        let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
            self.context.match_info.lock().unwrap();

        #[cfg(feature = "legacy_backend_full")]
        let sides_swapped: bool = !(msg.yellow_card_left || msg.red_card_left) && {
            msg.yellow_card_right || msg.red_card_right
        };

        #[cfg(feature = "legacy_backend_full")]
        if match_info_data.sides_swapped != sides_swapped {
            match_info_data.sides_swapped = sides_swapped;

            (match_info_data.left_fencer, match_info_data.right_fencer) = (
                match_info_data.right_fencer.clone(),
                match_info_data.left_fencer.clone(),
            );
        }

        Self::set_score(&mut match_info_data, msg);

        let reset_passive: bool = match_info_data.timer_controller.is_timer_running()
            && (msg.red || msg.white_red || msg.green || msg.white_green);

        if msg.symbol {
            let symbol: u32 = msg.dec_seconds * 16 + msg.seconds;

            self.auto_status_controller.set_state(match symbol {
                AUTO_STATUS_OFF => AutoStatusStates::Off,
                AUTO_STATUS_ON => AutoStatusStates::On,
                _ => AutoStatusStates::Unknown,
            });
        } else {
            let _update: bool = if let Some(mut prev_msg) = self.prev_uart_msg {
                prev_msg.minutes = msg.minutes;
                prev_msg.dec_seconds = msg.dec_seconds;
                prev_msg.seconds = msg.seconds;
                prev_msg.on_timer = msg.on_timer;

                prev_msg == msg
            } else {
                true
            };

            // if _update {
            Self::set_time(
                &mut match_info_data,
                msg,
                &mut self.prev_got_time,
                &mut self.last_second,
            );
            // }
        }

        Self::set_priority(&mut match_info_data, msg);

        if match_info_data.priority != match_info::Priority::None
            && match_info_data.timer_controller.get_main_time() <= Duration::from_secs(60)
        {
            Self::reset_passive_timer(&mut match_info_data);
        }

        match_info_data.left_fencer.color_light = msg.red;
        match_info_data.left_fencer.white_light = msg.white_red;
        match_info_data.right_fencer.color_light = msg.green;
        match_info_data.right_fencer.white_light = msg.white_green;

        if self.reset_passive || reset_passive {
            Self::reset_passive_timer(&mut match_info_data);
            self.reset_passive = false;
        }

        std::mem::drop(match_info_data);
        self.context.match_info_data_updated();

        self.prev_uart_msg = Some(msg);
    }

    #[cfg(feature = "legacy_backend_full")]
    fn apply_pins_data(&mut self, msg: PinsData) {
        let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
            self.context.match_info.lock().unwrap();

        let weapon: Weapon = match msg.weapon {
            3 => match_info::Weapon::Epee,
            1 => match_info::Weapon::Sabre,
            2 => match_info::Weapon::Fleuret,
            _ => match_info_data.weapon,
        };

        if weapon != match_info::Weapon::Epee {
            self.epee_5_counter = 0;
        }

        if match_info_data.weapon != weapon {
            match_info_data.weapon = weapon;
            match_info_data
                .timer_controller
                .set_passive_timer_active(weapon != match_info::Weapon::Sabre);
            std::mem::drop(match_info_data);
            self.context.match_info_data_updated();
        } else {
            std::mem::drop(match_info_data);
        }

        self.weapon_select_btn_pressed = msg.weapon_select_btn;
    }

    #[cfg(feature = "legacy_backend_full")]
    fn apply_ir_data(&mut self, msg: IrFrame) {
        if let Some(tx) = self.rc5_tx.as_ref() {
            if msg.address == self.rc5_address && msg.command.retranslate() {
                tx.send(IrFrame {
                    new: msg.new,
                    address: self.context.hw_config.legacy_backend.rc5_output_addr,
                    command: msg.command,
                })
                .log_err(&self.context.logger);
            }
        }

        if self.weapon_select_btn_pressed && msg.command == IrCommands::SetTime {
            self.rc5_address = msg.address;
            {
                {
                    let mut config: MutexGuard<'_, VirtuosoConfig> =
                        self.context.config.lock().unwrap();
                    config.legacy_backend.rc5_address = msg.address;
                    config.write_config().log_err(&self.context.logger);
                }

                let mut match_info_data: MutexGuard<'_, MatchInfo> =
                    self.context.match_info.lock().unwrap();

                match_info_data.display_message =
                    format!("Synced remote\nwith address {}", msg.address);
                match_info_data.display_message_updated =
                    Some(Instant::now() + Duration::from_secs(2));
            }
            self.context.match_info_data_updated();
        } else if msg.new && msg.address == self.rc5_address {
            let mut match_info_data: MutexGuard<'_, MatchInfo> =
                self.context.match_info.lock().unwrap();

            if match_info_data.timer_controller.is_medical_active() {
                if msg.command == IrCommands::TimerStartStop {
                    match_info_data.timer_controller.start_stop_medical();
                    std::mem::drop(match_info_data);
                    self.context.match_info_data_updated();
                } else if !match_info_data.timer_controller.is_timer_running()
                    && msg.command == IrCommands::Aux
                {
                    match_info_data.timer_controller.stop_medical_emergency();
                    std::mem::drop(match_info_data);
                    self.context.match_info_data_updated();
                }
                return;
            }
            std::mem::drop(match_info_data);

            if msg.command != IrCommands::Reset {
                self.epee_5_counter = 0;
            }

            match msg.command {
                IrCommands::AutoScoreOnOff => {
                    self.auto_status_controller
                        .set_field(AutoStatusFields::Score);
                }
                IrCommands::AutoTimerOnOff => {
                    self.auto_status_controller
                        .set_field(AutoStatusFields::Timer);
                }
                IrCommands::SetTime => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();

                    if !match_info_data.timer_controller.is_timer_running() {
                        match_info_data.priority = match_info::Priority::None;
                        self.reset_passive = true;
                    }
                }
                IrCommands::PeriodIncrement => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();

                    if !match_info_data.timer_controller.is_timer_running() {
                        match_info_data.period %= match match_info_data.competition_type {
                            None => 3,
                            Some(match_info::CompetitionType::Individual) => 3,
                            Some(match_info::CompetitionType::Team) => 9,
                        };
                        match_info_data.period += 1;

                        std::mem::drop(match_info_data);
                        self.context.match_info_data_updated();
                    }
                }
                IrCommands::PriorityRaffle => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();

                    if !match_info_data.timer_controller.is_timer_running() {
                        match_info_data.priority = match_info::Priority::None;
                        Self::reset_passive_timer(&mut match_info_data);
                    }
                }
                IrCommands::LeftPenaltyCard => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    if !match_info_data.timer_controller.is_timer_running() {
                        match_info_data.left_fencer.warning_card.inc();

                        std::mem::drop(match_info_data);
                        self.context.match_info_data_updated();
                    }
                }
                IrCommands::RightPenaltyCard => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    if !match_info_data.timer_controller.is_timer_running() {
                        match_info_data.right_fencer.warning_card.inc();
                        std::mem::drop(match_info_data);
                        self.context.match_info_data_updated();
                    }
                }
                IrCommands::LeftPassiveCard => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    if !match_info_data.timer_controller.is_timer_running() {
                        match_info_data.left_fencer.passive_card.inc();

                        Self::reset_passive_timer(&mut match_info_data);

                        std::mem::drop(match_info_data);
                        self.context.match_info_data_updated();
                    }
                }
                IrCommands::RightPassiveCard => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    if !match_info_data.timer_controller.is_timer_running() {
                        match_info_data.right_fencer.passive_card.inc();

                        Self::reset_passive_timer(&mut match_info_data);

                        std::mem::drop(match_info_data);
                        self.context.match_info_data_updated();
                    }
                }
                IrCommands::Prev => {
                    if self
                        .context
                        .settings_menu_shown
                        .load(std::sync::atomic::Ordering::Relaxed)
                    {
                        self.context.settings_menu.lock().unwrap().prev();
                    }
                }
                IrCommands::Next => {
                    if self
                        .context
                        .settings_menu_shown
                        .load(std::sync::atomic::Ordering::Relaxed)
                    {
                        self.context.settings_menu.lock().unwrap().next();
                    }
                }
                IrCommands::Begin => {
                    if self
                        .context
                        .settings_menu_shown
                        .load(std::sync::atomic::Ordering::Relaxed)
                    {
                        self.context
                            .settings_menu
                            .lock()
                            .unwrap()
                            .get_item_mut()
                            .prev();
                    }
                }
                IrCommands::End => {
                    if self
                        .context
                        .settings_menu_shown
                        .load(std::sync::atomic::Ordering::Relaxed)
                    {
                        self.context
                            .settings_menu
                            .lock()
                            .unwrap()
                            .get_item_mut()
                            .next();
                    }
                }
                IrCommands::Aux => {
                    if self
                        .context
                        .settings_menu_shown
                        .load(std::sync::atomic::Ordering::Relaxed)
                    {
                        self.context
                            .settings_menu
                            .lock()
                            .unwrap()
                            .get_item_mut()
                            .get_active_mut()
                            .press(
                                &self.context.logger,
                                self.context.settings_menu_shown.clone(),
                            );
                    } else {
                        self.context
                            .settings_menu_shown
                            .store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                }
                IrCommands::Reset => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    if !match_info_data.timer_controller.is_timer_running() {
                        match_info_data.right_fencer.passive_card = match_info::PassiveCard::None;
                        match_info_data.right_fencer.warning_card = match_info::WarningCard::None;
                        match_info_data.left_fencer.passive_card = match_info::PassiveCard::None;
                        match_info_data.left_fencer.warning_card = match_info::WarningCard::None;

                        match_info_data.priority = match_info::Priority::None;
                        match_info_data.period = 1;

                        self.reset_passive = true;

                        if !self.context.hw_config.legacy_backend.disable_epee_5 {
                            if match_info_data.weapon == Weapon::Epee {
                                self.epee_5_counter += 1;

                                if self.epee_5_counter == 5 {
                                    match_info_data.epee_5 = !match_info_data.epee_5;
                                    self.epee_5_counter = 0;
                                }
                            }
                        }

                        std::mem::drop(match_info_data);
                        self.context.match_info_data_updated();
                    }
                }
                IrCommands::LeftMedical | IrCommands::RightMedical => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();

                    if match_info_data.timer_controller.is_timer_running() {
                        match_info_data.timer_controller.start_stop(false);

                        if let Some(tx) = self.rc5_tx.as_ref() {
                            tx.send(IrFrame {
                                new: true,
                                address: self.context.hw_config.legacy_backend.rc5_output_addr,
                                command: IrCommands::TimerStartStop,
                            })
                            .log_err(&self.context.logger);
                        }
                    }

                    std::mem::drop(match_info_data);
                    self.context.match_info_data_updated();
                }
                _ => {}
            }

            if !self
                .context
                .settings_menu_shown
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                if let Some(cyrano_command) = msg.command.to_cyrano() {
                    self.context
                        .cyrano_command_tx
                        .send(cyrano_command)
                        .log_err(&self.context.logger);
                }
            }
        }
    }

    fn set_auto_statuses(&mut self) {
        self.context.logger.debug(format!(
            "Setting state of {:?} to {:?}",
            self.auto_status_controller.modified_field, self.auto_status_controller.new_state
        ));

        let (modified_field, new_state) = self.auto_status_controller.get_data();

        let new_state = match new_state {
            AutoStatusStates::Unknown => {
                return;
            }
            new_state => new_state,
        };

        let modified_field = match modified_field {
            AutoStatusFields::Unknown => {
                return;
            }
            modified_field => modified_field,
        };

        self.auto_status_controller.reset();

        self.context.logger.info(format!(
            "Setting state of {:?} to {:?}",
            modified_field, new_state
        ));

        let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
            self.context.match_info.lock().unwrap();

        match_info_data.display_message = format!("{} {}", modified_field, new_state);
        match_info_data.display_message_updated = Some(Instant::now() + Duration::from_secs(2));

        match modified_field {
            AutoStatusFields::Score => {
                match_info_data.auto_score_on = new_state.to_bool();
                std::mem::drop(match_info_data);
                self.context.match_info_data_updated();

                let mut config: MutexGuard<'_, VirtuosoConfig> =
                    self.context.config.lock().unwrap();
                config.legacy_backend.auto_score_on = new_state.to_bool();
                config.write_config().log_err(&self.context.logger);
            }
            AutoStatusFields::Timer => {
                match_info_data.auto_timer_on = new_state.to_bool();
                std::mem::drop(match_info_data);
                self.context.match_info_data_updated();

                let mut config: MutexGuard<'_, VirtuosoConfig> =
                    self.context.config.lock().unwrap();
                config.legacy_backend.auto_timer_on = new_state.to_bool();
                config.write_config().log_err(&self.context.logger);
            }
            _ => {}
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum AutoStatusFields {
    Timer,
    Score,
    Unknown,
}

impl std::fmt::Display for AutoStatusFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoStatusFields::Timer => write!(f, "Auto timer"),
            AutoStatusFields::Score => write!(f, "Auto score"),
            AutoStatusFields::Unknown => write!(f, ""),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum AutoStatusStates {
    On,
    Off,
    Unknown,
}

impl std::fmt::Display for AutoStatusStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoStatusStates::On => write!(f, "on"),
            AutoStatusStates::Off => write!(f, "off"),
            AutoStatusStates::Unknown => write!(f, ""),
        }
    }
}

impl AutoStatusStates {
    pub fn to_bool(&self) -> bool {
        match self {
            AutoStatusStates::On => true,
            AutoStatusStates::Off => false,
            AutoStatusStates::Unknown => false,
        }
    }
}

struct AutoStatusController {
    new_state: AutoStatusStates,
    modified_field: AutoStatusFields,

    previous_setting_state: std::time::Instant,
    previous_setting_field: std::time::Instant,
}

impl AutoStatusController {
    pub fn new() -> Self {
        Self {
            new_state: AutoStatusStates::Unknown,
            modified_field: AutoStatusFields::Unknown,

            previous_setting_state: std::time::Instant::now(),
            previous_setting_field: std::time::Instant::now(),
        }
    }

    pub fn get_data(&self) -> (AutoStatusFields, AutoStatusStates) {
        if self.previous_setting_field.elapsed() > AUTO_STATUS_WAIT_THRESHOLD
            || self.previous_setting_state.elapsed() > AUTO_STATUS_WAIT_THRESHOLD
        {
            (AutoStatusFields::Unknown, AutoStatusStates::Unknown)
        } else {
            (self.modified_field, self.new_state)
        }
    }

    pub fn reset(&mut self) {
        self.new_state = AutoStatusStates::Unknown;
        self.modified_field = AutoStatusFields::Unknown;
    }

    pub fn set_state(&mut self, new_state: AutoStatusStates) {
        self.new_state = new_state;
        self.previous_setting_state = std::time::Instant::now();
    }

    pub fn set_field(&mut self, modified_field: AutoStatusFields) {
        self.modified_field = modified_field;
        self.previous_setting_field = std::time::Instant::now();
    }
}

enum InputData {
    UartData(UartData),
    #[cfg(feature = "legacy_backend_full")]
    PinsData(PinsData),
    #[cfg(feature = "legacy_backend_full")]
    IrCommand(IrFrame),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct UartData {
    red: bool,
    white_red: bool,
    #[allow(dead_code)]
    yellow_red: bool,

    green: bool,
    white_green: bool,
    #[allow(dead_code)]
    yellow_green: bool,

    #[allow(dead_code)]
    apparel_sound: bool,

    symbol: bool,

    on_timer: bool,

    minutes: u32,
    dec_seconds: u32,
    seconds: u32,

    #[allow(dead_code)]
    timer_sound: bool,
    score_left: u32,
    score_right: u32,
    period: u32,

    #[allow(dead_code)]
    yellow_card_left: bool,
    #[allow(dead_code)]
    red_card_left: bool,
    #[allow(dead_code)]
    yellow_card_right: bool,
    #[allow(dead_code)]
    red_card_right: bool,
}

impl UartData {
    fn from_8bytes(src: [u8; 8]) -> Self {
        UartData {
            red: src[0] >> 0 & 1 == 1,
            white_red: src[0] >> 2 & 1 == 1,
            yellow_red: src[0] >> 1 & 1 == 1,

            green: src[0] >> 3 & 1 == 1,
            white_green: src[1] >> 4 & 1 == 1,
            yellow_green: src[0] >> 4 & 1 == 1,

            apparel_sound: src[1] >> 3 & 1 == 1,
            symbol: src[1] >> 2 & 1 == 1,
            on_timer: src[2] >> 4 & 1 == 1,
            timer_sound: src[3] >> 4 & 1 == 1,

            score_left: (((src[6] & 0b00010000) << 1) | (src[4] & 0b00011111)) as u32,
            score_right: (((src[7] & 0b00010000) << 1) | (src[5] & 0b00011111)) as u32,

            minutes: (src[1] & 0b11) as u32,
            dec_seconds: (src[2] & 0b00001111) as u32,
            seconds: (src[3] & 0b00001111) as u32,

            period: (src[6] & 0b00001111) as u32,

            yellow_card_left: (src[7] >> 2 & 0b00000001) == 1,
            red_card_left: (src[7] >> 3 & 0b00000001) == 1,
            yellow_card_right: (src[7] >> 0 & 0b00000001) == 1,
            red_card_right: (src[7] >> 1 & 0b00000001) == 1,
        }
    }
}

fn uart_handler(tx: mpsc::SyncSender<InputData>, port: serialport::TTYPort) {
    let mut buf: [u8; 8] = [0; 8];
    let mut ind: usize = 0;

    let mut prev_msg: Option<UartData> = None;

    for byte in port.bytes() {
        match byte {
            Err(_) => {
                thread::sleep(Duration::from_millis(100));
            }
            Ok(byte_val) => {
                if byte_val >> 5 == 0 {
                    ind = 0;
                }

                if byte_val >> 5 == ind as u8 {
                    buf[ind] = byte_val;
                    ind += 1;

                    if ind == 8 {
                        ind = 0;

                        let data: UartData = UartData::from_8bytes(buf);

                        if Some(data) != prev_msg {
                            tx.send(InputData::UartData(data)).unwrap();
                        } else {
                            eprintln!("Got uart packet duplicate");
                        }

                        prev_msg = Some(data);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum IrCommands {
    TimerStartStop,

    AutoTimerOnOff,
    AutoScoreOnOff,

    LeftScoreIncrement,
    LeftScoreDecrement,
    RightScoreIncrement,
    RightScoreDecrement,

    LeftPassiveCard,
    RightPassiveCard,

    LeftPenaltyCard,
    RightPenaltyCard,

    SecondsIncrement,
    SecondsDecrement,

    PriorityRaffle,

    SetTime,
    FlipSides,

    ChangeWeapon,

    Reset,

    PeriodIncrement,

    Prev,
    Next,
    End,
    Begin,

    LeftStatus,
    RightStatus,
    LeftVideoAppeal,
    RightVideoAppeal,
    LeftMedical,
    RightMedical,
    LeftReserve,
    RightReserve,

    Aux,

    Unknown(u32),
}

impl IrCommands {
    pub fn from_u32(command: u32) -> Self {
        match command {
            13 => IrCommands::TimerStartStop,

            1 => IrCommands::AutoTimerOnOff,
            16 => IrCommands::AutoScoreOnOff,

            2 => IrCommands::LeftScoreIncrement,
            3 => IrCommands::LeftScoreDecrement,
            9 => IrCommands::RightScoreIncrement,
            15 => IrCommands::RightScoreDecrement,

            17 => IrCommands::LeftPassiveCard,
            18 => IrCommands::RightPassiveCard,

            4 => IrCommands::LeftPenaltyCard,
            11 => IrCommands::RightPenaltyCard,

            14 => IrCommands::SecondsIncrement,
            6 => IrCommands::SecondsDecrement,

            12 => IrCommands::PriorityRaffle,

            7 => IrCommands::SetTime,
            0 => IrCommands::FlipSides,

            5 => IrCommands::ChangeWeapon,

            10 => IrCommands::Reset,

            8 => IrCommands::PeriodIncrement,

            21 => IrCommands::Prev,
            20 => IrCommands::Next,
            19 => IrCommands::Begin,
            24 => IrCommands::End,
            38 => IrCommands::Aux,

            25 => IrCommands::LeftStatus,
            26 => IrCommands::RightStatus,
            27 => IrCommands::LeftVideoAppeal,
            28 => IrCommands::RightVideoAppeal,
            29 => IrCommands::LeftMedical,
            30 => IrCommands::RightMedical,
            31 => IrCommands::LeftReserve,
            37 => IrCommands::RightReserve,

            unknown => IrCommands::Unknown(unknown),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            IrCommands::TimerStartStop => 13,

            IrCommands::AutoTimerOnOff => 1,
            IrCommands::AutoScoreOnOff => 16,

            IrCommands::LeftScoreIncrement => 2,
            IrCommands::LeftScoreDecrement => 3,
            IrCommands::RightScoreIncrement => 9,
            IrCommands::RightScoreDecrement => 15,

            IrCommands::LeftPassiveCard => 17,
            IrCommands::RightPassiveCard => 18,

            IrCommands::LeftPenaltyCard => 4,
            IrCommands::RightPenaltyCard => 11,

            IrCommands::SecondsIncrement => 14,
            IrCommands::SecondsDecrement => 6,

            IrCommands::PriorityRaffle => 12,

            IrCommands::SetTime => 7,
            IrCommands::FlipSides => 0,

            IrCommands::ChangeWeapon => 5,

            IrCommands::Reset => 10,

            IrCommands::PeriodIncrement => 8,

            IrCommands::Prev => 21,
            IrCommands::Next => 20,
            IrCommands::Begin => 19,
            IrCommands::End => 24,
            IrCommands::Aux => 38,

            IrCommands::LeftStatus => 25,
            IrCommands::RightStatus => 26,
            IrCommands::LeftVideoAppeal => 27,
            IrCommands::RightVideoAppeal => 28,
            IrCommands::LeftMedical => 29,
            IrCommands::RightMedical => 30,
            IrCommands::LeftReserve => 31,
            IrCommands::RightReserve => 37,

            IrCommands::Unknown(unknown) => *unknown,
        }
    }

    pub fn retranslate(&self) -> bool {
        match self {
            IrCommands::TimerStartStop => true,

            IrCommands::AutoTimerOnOff => true,
            IrCommands::AutoScoreOnOff => true,

            IrCommands::LeftScoreIncrement => true,
            IrCommands::LeftScoreDecrement => true,
            IrCommands::RightScoreIncrement => true,
            IrCommands::RightScoreDecrement => true,

            IrCommands::SecondsIncrement => true,
            IrCommands::SecondsDecrement => true,

            IrCommands::PriorityRaffle => true,

            IrCommands::SetTime => true,
            IrCommands::FlipSides => true,

            IrCommands::ChangeWeapon => true,

            IrCommands::Reset => true,

            _ => false,
        }
    }

    pub fn to_cyrano(&self) -> Option<CyranoCommand> {
        match self {
            Self::Next => Some(CyranoCommand::CyranoNext),
            Self::Prev => Some(CyranoCommand::CyranoPrev),
            Self::Begin => Some(CyranoCommand::CyranoBegin),
            Self::End => Some(CyranoCommand::CyranoEnd),

            Self::LeftStatus => Some(CyranoCommand::LeftStatus),
            Self::RightStatus => Some(CyranoCommand::RightStatus),
            Self::LeftVideoAppeal => Some(CyranoCommand::LeftVideoAppeal),
            Self::RightVideoAppeal => Some(CyranoCommand::RightVideoAppeal),
            Self::LeftMedical => Some(CyranoCommand::LeftMedical),
            Self::RightMedical => Some(CyranoCommand::RightMedical),
            Self::LeftReserve => Some(CyranoCommand::LeftReserve),
            Self::RightReserve => Some(CyranoCommand::RightReserve),

            _ => None,
        }
    }
}

#[cfg(feature = "legacy_backend_full")]
#[derive(Debug)]
struct IrFrame {
    new: bool,
    address: u32,
    command: IrCommands,
}

#[cfg(feature = "legacy_backend_full")]
impl IrFrame {
    pub fn from_buf(buf: [u32; 14], new: bool) -> Self {
        let mut address: u32 = 0;
        let mut command: u32 = 0;

        for i in 3..8 {
            address *= 2;
            address += buf[i];
        }

        for i in 8..14 {
            command *= 2;
            command += buf[i];
        }

        Self {
            new,
            address,
            command: IrCommands::from_u32(command),
        }
    }
}

#[cfg(feature = "legacy_backend_full")]
fn rc5_transmitter(rx: mpsc::Receiver<IrFrame>, logger: Logger, line: gpio_cdev::Line) {
    let handler: gpio_cdev::LineHandle = line
        .request(gpio_cdev::LineRequestFlags::OUTPUT, 1, "rc5_tx")
        .unwrap();

    let mut toggle_bit: bool = true;

    loop {
        let frame: IrFrame = rx.recv().unwrap();
        logger.debug(format!("Transmitting ir: {:?}", frame));
        toggle_bit = if frame.new { !toggle_bit } else { toggle_bit };

        let mut buf: [u8; 14] = [0; 14];

        buf[0] = 1;
        buf[1] = 1;
        buf[2] = if toggle_bit { 1 } else { 0 };

        for i in 0..5 {
            buf[7 - i] = (frame.address as u8 >> i) & 1;
        }

        for i in 0..6 {
            buf[13 - i] = (frame.command.to_u32() as u8 >> i) & 1;
        }

        for i in 0..14 {
            let time: Instant = Instant::now();
            handler.set_value(buf[i]).log_err(&logger);
            while time.elapsed() < Duration::from_micros(889) {}
            let time: Instant = Instant::now();
            handler.set_value(1 - buf[i]).log_err(&logger);
            while time.elapsed() < Duration::from_micros(889) {}
        }
        handler.set_value(1).log_err(&logger);
        logger.debug(format!("Transmitted buffer: {:?}", buf));
        thread::sleep(Duration::from_micros(889 * 50));
    }
}

#[cfg(feature = "legacy_backend_full")]
fn rc5_receiever(tx: mpsc::SyncSender<InputData>, logger: Logger, line: gpio_cdev::Line) {
    let mut last_interrupt_time: u64 = 0u64;

    let mut receieve_buf: [u32; 14] = [1; 14];
    let mut index: usize = 1;
    let mut last_toggle_value: u32 = 2;

    for event in line
        .events(
            gpio_cdev::LineRequestFlags::INPUT,
            gpio_cdev::EventRequestFlags::BOTH_EDGES,
            "read ir remote",
        )
        .unwrap()
    {
        let event: gpio_cdev::LineEvent = event.unwrap();
        let event_delta: u64 = event.timestamp() - last_interrupt_time;
        last_interrupt_time = event.timestamp();

        if event_delta > 889 * 1000 * 3 {
            index = 1;
            receieve_buf = [1; 14];
            continue;
        }

        let delta: i32 = if event_delta > 889 * 1000 * 3 / 2 {
            2
        } else {
            1
        };

        let next_value: Option<u32> = match (receieve_buf[index - 1], event.event_type(), delta) {
            (1, gpio_cdev::EventType::RisingEdge, 1) => Some(1),
            (1, gpio_cdev::EventType::RisingEdge, 2) => Some(0),
            (0, gpio_cdev::EventType::FallingEdge, 1) => Some(0),
            (0, gpio_cdev::EventType::FallingEdge, 2) => Some(1),
            _ => None,
        };

        if let Some(next_value) = next_value {
            receieve_buf[index] = next_value;
            index += 1;

            if index == 14 {
                let toggle_bit: u32 = receieve_buf[2];

                logger.debug(format!("Got ir packet: {receieve_buf:?}"));

                let frame: IrFrame =
                    IrFrame::from_buf(receieve_buf, toggle_bit != last_toggle_value);

                tx.send(InputData::IrCommand(frame)).log_err(&logger);

                index = 1;
                last_toggle_value = toggle_bit;
            }
        }
    }
}

#[cfg(feature = "legacy_backend_full")]
#[derive(Clone, Copy, Debug, PartialEq)]
struct PinsData {
    weapon: u8,
    weapon_select_btn: bool,
}

#[cfg(feature = "legacy_backend_full")]
fn pins_handler(
    tx: mpsc::SyncSender<InputData>,
    logger: Logger,
    gpio_line_weapon_0: Line,
    gpio_line_weapon_1: Line,
    gpio_line_weapon_btn: Line,
) {
    let gpio_handle_weapon_0: gpio_cdev::LineHandle = gpio_line_weapon_0
        .request(gpio_cdev::LineRequestFlags::INPUT, 0, "read weapon 1")
        .unwrap_with_logger(&logger);
    let gpio_handle_weapon_1: gpio_cdev::LineHandle = gpio_line_weapon_1
        .request(gpio_cdev::LineRequestFlags::INPUT, 0, "read weapon 2")
        .unwrap_with_logger(&logger);

    let gpio_handle_weapon_btn: gpio_cdev::LineHandle = gpio_line_weapon_btn
        .request(gpio_cdev::LineRequestFlags::INPUT, 0, "read weapon button")
        .unwrap_with_logger(&logger);

    let mut old_pins_data: Option<PinsData> = Option::None;

    loop {
        let new_pins_data: PinsData = PinsData {
            weapon: gpio_handle_weapon_0.get_value().unwrap_with_logger(&logger) * 2
                + gpio_handle_weapon_1.get_value().unwrap_with_logger(&logger),
            weapon_select_btn: gpio_handle_weapon_btn
                .get_value()
                .unwrap_with_logger(&logger)
                == 0u8,
        };

        if old_pins_data.as_ref() != Some(&new_pins_data) {
            tx.send(InputData::PinsData(new_pins_data.clone()))
                .unwrap_with_logger(&logger);
        }

        old_pins_data = Some(new_pins_data);

        thread::sleep(Duration::from_millis(10));
    }
}
