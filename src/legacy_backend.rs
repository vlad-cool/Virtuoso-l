use gpio_cdev;
use gpio_cdev::Line;
use serial::{self, SerialPort};
use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc::RecvError;
use std::sync::{MutexGuard, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use crate::match_info;
use crate::modules::{self, VirtuosoModuleContext};
use crate::virtuoso_config::VirtuosoConfig;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

const AUTO_STATUS_WAIT_THRESHOLD: std::time::Duration = std::time::Duration::from_millis(200);
const AUTO_STATUS_ON: u32 = 196;
const AUTO_STATUS_OFF: u32 = 17;

pub struct LegacyBackend {
    context: VirtuosoModuleContext,

    weapon_select_btn_pressed: bool,
    rc5_address: u32,
    auto_status_controller: AutoStatusController,
}

impl modules::VirtuosoModule for LegacyBackend {
    fn run(mut self) {
        let (tx, rx) = mpsc::channel::<InputData>();

        let tx_clone: mpsc::Sender<InputData> = tx.clone();
        let logger_clone: Logger = self.context.logger.clone();
        let port_path: PathBuf = self.context.hw_config.legacy_backend.uart_port.clone();
        thread::spawn(move || {
            uart_handler(tx_clone, logger_clone, port_path);
        });

        let tx_clone: mpsc::Sender<InputData> = tx.clone();
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

        let tx_clone: mpsc::Sender<InputData> = tx.clone();
        let logger_clone: Logger = self.context.logger.clone();
        let ir_line: Line = self
            .context
            .hw_config
            .legacy_backend
            .ir_pin
            .to_line()
            .unwrap_with_logger(&self.context.logger);
        thread::spawn(move || {
            rc5_receiever(tx_clone, logger_clone, ir_line);
        });

        loop {
            match rx.recv() {
                Err(RecvError) => {}
                Ok(msg) => {
                    match msg {
                        InputData::UartData(msg) => {
                            self.apply_uart_data(msg);
                        }
                        InputData::PinsData(msg) => {
                            self.apply_pins_data(msg);
                        }
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
        let rc5_address: u32 = context.config.lock().unwrap().legacy_backend.rc5_address;
        Self {
            context,

            weapon_select_btn_pressed: false,
            rc5_address,
            auto_status_controller: AutoStatusController::new(),
        }
    }

    fn apply_uart_data(&mut self, msg: UartData) {
        let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
            self.context.match_info.lock().unwrap();

        match_info_data.left_fencer.score = msg.score_left;
        match_info_data.right_fencer.score = msg.score_right;
        match_info_data.timer_running = msg.on_timer;

        if msg.symbol {
            let symbol: u32 = msg.dec_seconds * 16 + msg.seconds;

            self.auto_status_controller.set_state(match symbol {
                AUTO_STATUS_OFF => AutoStatusStates::Off,
                AUTO_STATUS_ON => AutoStatusStates::On,
                _ => AutoStatusStates::Unknown,
            });
        } else {
            let timer_m: u32 = if msg.period == 0b1100 { 4 } else { msg.minutes };
            let timer_d: u32 = msg.dec_seconds;
            let timer_s: u32 = msg.seconds;

            match_info_data
                .timer_controller
                .set_time(timer_m, timer_d, timer_s);
            if msg.on_timer {
                match_info_data.timer_controller.start_timer();
                match_info_data.passive_timer.enable();
            } else {
                match_info_data.timer_controller.stop_timer();
                match_info_data.passive_timer.disable();
            }

            if match_info_data.timer_controller.get_second_changed() {
                match_info_data.passive_timer.tick();
            }
        }
        match_info_data.period = if msg.period > 0 && msg.period < 10 {
            msg.period
        } else {
            match_info_data.period
        };
        match_info_data.priority = match msg.period {
            0b1110 => match_info::Priority::Right,
            0b1111 => match_info::Priority::Left,
            0b1011 => match_info::Priority::None,
            _ => match match_info_data.priority {
                match_info::Priority::Right => match_info::Priority::Right,
                match_info::Priority::Left => match_info::Priority::Left,
                match_info::Priority::None => match_info::Priority::None,
            },
        };

        // match_info_data.left_fencer.yellow_card =
        //     (msg.yellow_card_left || msg.red_card_left) as u32;
        // match_info_data.left_fencer.red_card = msg.red_card_left as u32;
        // match_info_data.right_fencer.yellow_card =
        //     (msg.yellow_card_right || msg.red_card_right) as u32;
        // match_info_data.right_fencer.red_card = msg.red_card_right as u32;

        match_info_data.left_fencer.color_light = msg.red;
        match_info_data.left_fencer.white_light = msg.white_red;
        match_info_data.right_fencer.color_light = msg.green;
        match_info_data.right_fencer.white_light = msg.white_green;

        if msg.red || msg.white_red || msg.green || msg.white_green {
            match_info_data.passive_timer.reset();
        }

        // match_info_data.modified_count += 1;
        std::mem::drop(match_info_data);
        self.context
            .match_info_modified_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn apply_pins_data(&mut self, msg: PinsData) {
        let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
            self.context.match_info.lock().unwrap();

        match_info_data.weapon = match msg.weapon {
            3 => match_info::Weapon::Epee,
            1 => match_info::Weapon::Sabre,
            2 => match_info::Weapon::Fleuret,
            _ => match_info_data.weapon,
        };

        std::mem::drop(match_info_data);
        self.context
            .match_info_modified_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.weapon_select_btn_pressed = msg.weapon_select_btn;
    }

    fn apply_ir_data(&mut self, msg: IrFrame) {
        if self.weapon_select_btn_pressed
            && msg.address != self.rc5_address
            && msg.command == IrCommands::SetTime
        {
            self.rc5_address = msg.address;
            let mut config: MutexGuard<'_, VirtuosoConfig> = self.context.config.lock().unwrap();
            config.legacy_backend.rc5_address = msg.address;
            config.write_config();
        } else if msg.new && msg.address == self.rc5_address {
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

                    if !match_info_data.timer_running {
                        match_info_data.passive_timer.reset();
                    }
                }
                IrCommands::LeftPenaltyCard => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    match_info_data.left_fencer.warning_card.inc();

                    std::mem::drop(match_info_data);
                    self.context
                        .match_info_modified_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                IrCommands::RightPenaltyCard => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    match_info_data.right_fencer.warning_card.inc();

                    std::mem::drop(match_info_data);
                    self.context
                        .match_info_modified_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                IrCommands::LeftPassiveCard => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    match_info_data.left_fencer.passive_card.inc();

                    std::mem::drop(match_info_data);
                    self.context
                        .match_info_modified_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                IrCommands::RightPassiveCard => {
                    let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                        self.context.match_info.lock().unwrap();
                    match_info_data.right_fencer.passive_card.inc();

                    std::mem::drop(match_info_data);
                    self.context
                        .match_info_modified_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                // IrCommands::FlipSides => {
                //     let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
                //         self.match_info.lock().unwrap();

                //     match_info_data.
                // }
                _ => {}
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

        match modified_field {
            AutoStatusFields::Score => match_info_data.auto_score_on = new_state.to_bool(),
            AutoStatusFields::Timer => match_info_data.auto_timer_on = new_state.to_bool(),
            _ => {}
        }

        match_info_data.display_message = format!("{} {}", modified_field, new_state);
        match_info_data.display_message_updated = Instant::now();

        std::mem::drop(match_info_data);
        self.context
            .match_info_modified_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
    PinsData(PinsData),
    IrCommand(IrFrame),
}

#[derive(Debug)]
struct UartData {
    #[allow(dead_code)]
    yellow_red: bool,
    white_red: bool,
    red: bool,

    #[allow(dead_code)]
    yellow_green: bool,
    white_green: bool,
    green: bool,

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
            yellow_red: src[0] >> 4 & 1 == 1,
            red: src[0] >> 3 & 1 == 1,
            white_green: src[0] >> 2 & 1 == 1,
            yellow_green: src[0] >> 1 & 1 == 1,
            green: src[0] >> 0 & 1 == 1,
            white_red: src[1] >> 4 & 1 == 1,
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

fn uart_handler(tx: mpsc::Sender<InputData>, logger: Logger, port_path: PathBuf) {
    let mut port: serial::unix::TTYPort = match serial::open(&port_path) {
        Ok(port) => port,
        Err(err) => {
            logger.critical_error(format!("Failed to open uart port, error: {err}"));
            return;
        }
    };

    let settings: serial::PortSettings = serial::PortSettings {
        baud_rate: serial::BaudRate::Baud38400,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };

    match port.configure(&settings) {
        Ok(()) => {}
        Err(err) => {
            logger.critical_error(format!("Failed to configure uart port, error: {err}"));
            return;
        }
    }

    match port.set_timeout(Duration::from_secs(60)) {
        Ok(()) => {}
        Err(err) => {
            logger.critical_error(format!("Failed to set uart port timeout, error: {err}"));
            return;
        }
    }

    let mut buf: [u8; 8] = [0; 8];
    let mut ind: usize = 0;

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

                        tx.send(InputData::UartData(UartData::from_8bytes(buf)))
                            .unwrap();
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
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

    Unknown,
}

impl IrCommands {
    pub fn from_int(command: u32) -> Self {
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

            _ => IrCommands::Unknown,
        }
    }
}

#[derive(Debug)]
struct IrFrame {
    new: bool,
    address: u32,
    command: IrCommands,
}

fn rc5_receiever(tx: mpsc::Sender<InputData>, logger: Logger, line: gpio_cdev::Line) {
    let mut last_interrupt_time: u64 = 0u64;

    let mut receieve_buf: [i32; 28] = [0; 28];
    let mut index: usize = 0;

    let mut last_toggle_value: i32 = -1;

    for event in line
        .events(
            gpio_cdev::LineRequestFlags::INPUT,
            gpio_cdev::EventRequestFlags::BOTH_EDGES,
            "read ir remote",
        )
        .unwrap()
    {
        let event: gpio_cdev::LineEvent = event.unwrap();

        let val: i32 = match event.event_type() {
            gpio_cdev::EventType::RisingEdge => 0,
            gpio_cdev::EventType::FallingEdge => 1,
        };
        let mut count: i32 = 0;

        if event.timestamp() - last_interrupt_time > 889 * 1000 * 5 / 2 {
            receieve_buf[0] = val;
            index = 1;
            count = 0;
        } else if event.timestamp() - last_interrupt_time > 889 * 1000 * 3 / 2 {
            count = 2;
        } else if event.timestamp() - last_interrupt_time > 889 * 1000 * 1 / 2 {
            count = 1;
        }

        for _ in 0..count {
            receieve_buf[index] = val;
            index += 1;

            if index == 27 {
                receieve_buf[index] = 1 - val;
                index += 1;
            }

            if index == 28 {
                for i in 0..14 {
                    if receieve_buf[i * 2] + receieve_buf[i * 2 + 1] != 1 {
                        logger.error("rc 5 receiver got bad buffer".to_string());
                        index = 0;
                        break;
                    }
                }
            }

            if index == 28 {
                let rc5_frame: Vec<i32> = receieve_buf.iter().step_by(2).cloned().collect();

                let toggle_bit = rc5_frame[2];

                let mut address = 0;
                let mut command = 0;

                for i in 3..8 {
                    address *= 2;
                    address += rc5_frame[i];
                }

                for i in 8..14 {
                    command *= 2;
                    command += rc5_frame[i];
                }

                tx.send(InputData::IrCommand(IrFrame {
                    new: toggle_bit != last_toggle_value,
                    address: address as u32,
                    command: IrCommands::from_int(command as u32),
                }))
                .unwrap();

                last_toggle_value = toggle_bit;

                index = 0;
                break;
            }
        }

        last_interrupt_time = event.timestamp();
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PinsData {
    weapon: u8,
    weapon_select_btn: bool,
}

fn pins_handler(
    tx: mpsc::Sender<InputData>,
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
