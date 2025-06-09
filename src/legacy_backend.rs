#[allow(dead_code)]
#[allow(unused_variables)]
use gpio_cdev;
use serial::{self, SerialPort};
use std::sync::mpsc::RecvError;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::{io::Read, sync::mpsc};

/*
TODO Passive counter / indicator
TODO Auto statuses
TODO Leds
*/

use crate::match_info;
use crate::modules;
use crate::virtuoso_config::VirtuosoConfig;

const AUTO_STATUS_WAIT_THRESHOLD: std::time::Duration = std::time::Duration::from_millis(200);
const AUTO_STATUS_ON: u32 = 196;
const AUTO_STATUS_OFF: u32 = 17;

pub struct LegacyBackend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    config: Arc<Mutex<VirtuosoConfig>>,
    weapon_select_btn_pressed: bool,
    rc5_address: u32,
    auto_status_controller: AutoStatusController,
    passive_controller: PassiveTimer,

    last_seconds_value: Option<u32>,
}

impl modules::VirtuosoModule for LegacyBackend {
    fn run(&mut self) {
        let (tx, rx) = mpsc::channel::<InputData>();

        let tx_cloned = tx.clone();
        thread::spawn(move || {
            uart_handler(tx_cloned);
        });

        let tx_cloned = tx.clone();
        thread::spawn(move || {
            pins_handler(tx_cloned);
        });

        let tx_cloned = tx.clone();
        thread::spawn(move || {
            rc5_reciever(tx_cloned);
        });

        loop {
            match rx.recv() {
                Err(RecvError) => {}
                Ok(msg) => match msg {
                    InputData::UartData(msg) => {
                        self.apply_uart_data(msg);
                    }
                    InputData::PinsData(msg) => {
                        self.apply_pins_data(msg);
                    }
                    InputData::IrCommand(msg) => {
                        self.apply_ir_data(msg);
                    }
                },
            }
        }
    }

    fn get_module_type(&self) -> Modules {
        modules::Modules::LegacyBackend
    }
}

impl LegacyBackend {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        config: Arc<Mutex<VirtuosoConfig>>,
    ) -> Self {
        let rc5_address: u32 = config.lock().unwrap().legacy_backend.rc5_address;
        Self {
            match_info: Arc::clone(&match_info),
            config,
            weapon_select_btn_pressed: false,
            rc5_address,
            auto_status_controller: AutoStatusController::new(),
            passive_controller: PassiveTimer::new(),
            last_seconds_value: None,
        }
    }

    fn apply_uart_data(&mut self, msg: UartData) {
        let mut match_info_data = self.match_info.lock().unwrap();

        match_info_data.left_score = msg.score_left;
        match_info_data.right_score = msg.score_right;
        match_info_data.timer_running = msg.on_timer;

        if msg.symbol {
            let symbol = msg.dec_seconds * 16 + msg.seconds;

            let (modified_field, new_state) = self.auto_status_controller.set_state(match symbol {
                AUTO_STATUS_OFF => AutoStatusStates::Off,
                AUTO_STATUS_ON => AutoStatusStates::On,
                _ => AutoStatusStates::Unknown,
            });

            if new_state != AutoStatusStates::Unknown {
                match modified_field {
                    AutoStatusFields::Timer => match_info_data.auto_timer_on = new_state.to_bool(),
                    AutoStatusFields::Score => match_info_data.auto_score_on = new_state.to_bool(),
                    AutoStatusFields::Unknown => {}
                }
            }
        } else {
            match_info_data.timer = if msg.period == 0b1100 { 4 } else { msg.minutes } * 100
                + msg.dec_seconds * 10
                + msg.seconds;

            if msg.minutes == 0 && msg.dec_seconds == 0 {
                match_info_data.last_ten_seconds = true;
            } else if msg.minutes != 0 {
                match_info_data.last_ten_seconds = false;
            }

            let secs = if match_info_data.last_ten_seconds {
                msg.minutes
            } else {
                msg.seconds
            };
            // let secs = msg.seconds;

            // if secs == None {
            //     secs = Some(self.last_seconds_value);
            // }

            // if secs != self.last_seconds_value.unwrap_or(11111) {
            //     self.last_seconds_value = Some(secs);
            //     self.passive_controller.tick();
            // }
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

        match_info_data.left_caution = msg.yellow_card_left || msg.red_card_left;
        match_info_data.left_penalty = msg.red_card_left;
        match_info_data.right_caution = msg.yellow_card_right || msg.red_card_right;
        match_info_data.right_penalty = msg.red_card_right;

        match_info_data.modified_count += 1;
    }

    fn apply_pins_data(&mut self, msg: PinsData) {
        let mut match_info_data = self.match_info.lock().unwrap();

        match_info_data.weapon = match msg.weapon {
            3 => match_info::Weapon::Epee,
            1 => match_info::Weapon::Sabre,
            2 => match_info::Weapon::Fleuret,
            _ => match_info::Weapon::Unknown,
        };

        match_info_data.modified_count += 1;

        self.weapon_select_btn_pressed = msg.weapon_select_btn;
    }

    fn apply_ir_data(&mut self, msg: IrFrame) {
        if self.weapon_select_btn_pressed
            && msg.address != self.rc5_address
            && msg.command == IrCommands::SetTime
        {
            self.rc5_address = msg.address;
            let mut config = self.config.lock().unwrap();
            config.legacy_backend.rc5_address = msg.address;
            config.write_config(None);
        } else if msg.new && msg.address == self.rc5_address {
            match msg.command {
                IrCommands::LeftPassiveCard => {
                    let mut match_info_data = self.match_info.lock().unwrap();

                    (
                        match_info_data.left_pcard_bot,
                        match_info_data.left_pcard_top,
                    ) = match (
                        match_info_data.left_pcard_bot,
                        match_info_data.left_pcard_top,
                    ) {
                        (false, false) => (true, false),
                        (true, false) => (true, true),
                        (false, true) => (true, true),
                        (true, true) => (false, false),
                    };

                    match_info_data.modified_count += 1;
                }
                IrCommands::RightPassiveCard => {
                    let mut match_info_data = self.match_info.lock().unwrap();

                    (
                        match_info_data.right_pcard_bot,
                        match_info_data.right_pcard_top,
                    ) = match (
                        match_info_data.right_pcard_bot,
                        match_info_data.right_pcard_top,
                    ) {
                        (false, false) => (true, false),
                        (true, false) => (true, true),
                        (false, true) => (true, true),
                        (true, true) => (false, false),
                    };

                    match_info_data.modified_count += 1;
                }
                IrCommands::AutoScoreOnOff => {
                    let mut match_info_data = self.match_info.lock().unwrap();

                    let (modified_field, new_state) = self
                        .auto_status_controller
                        .set_field(AutoStatusFields::Score);

                    if modified_field != AutoStatusFields::Unknown
                        && new_state != AutoStatusStates::Unknown
                    {
                        match modified_field {
                            AutoStatusFields::Timer => {
                                match_info_data.auto_timer_on = new_state.to_bool()
                            }
                            AutoStatusFields::Score => {
                                match_info_data.auto_score_on = new_state.to_bool()
                            }
                            _ => {}
                        }
                    };

                    match_info_data.modified_count += 1;
                }
                IrCommands::AutoTimerOnOff => {
                    let mut match_info_data = self.match_info.lock().unwrap();

                    let (modified_field, new_state) = self
                        .auto_status_controller
                        .set_field(AutoStatusFields::Timer);

                    if modified_field != AutoStatusFields::Unknown
                        && new_state != AutoStatusStates::Unknown
                    {
                        match modified_field {
                            AutoStatusFields::Timer => {
                                match_info_data.auto_timer_on = new_state.to_bool()
                            }
                            AutoStatusFields::Score => {
                                match_info_data.auto_score_on = new_state.to_bool()
                            }
                            _ => {}
                        }
                    }

                    match_info_data.modified_count += 1;
                }
                _ => {}
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum AutoStatusFields {
    Timer,
    Score,
    Unknown,
}

#[derive(PartialEq, Debug, Clone)]
enum AutoStatusStates {
    On,
    Off,
    Unknown,
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

    fn return_new_status(&mut self) -> (AutoStatusFields, AutoStatusStates) {
        let ret_val = (self.modified_field.clone(), self.new_state.clone());

        println!("{:?} - {:?}", self.modified_field, self.new_state);

        self.new_state = AutoStatusStates::Unknown;
        self.modified_field = AutoStatusFields::Unknown;

        ret_val
    }

    pub fn set_state(
        &mut self,
        new_state: AutoStatusStates,
    ) -> (AutoStatusFields, AutoStatusStates) {
        self.new_state = new_state;
        self.previous_setting_state = std::time::Instant::now();
        if self.new_state != AutoStatusStates::Unknown
            && self.modified_field != AutoStatusFields::Unknown
            && self.previous_setting_field.elapsed() < AUTO_STATUS_WAIT_THRESHOLD
        {
            return self.return_new_status();
        } else {
            return (AutoStatusFields::Unknown, AutoStatusStates::Unknown);
        }
    }

    pub fn set_field(
        &mut self,
        modified_field: AutoStatusFields,
    ) -> (AutoStatusFields, AutoStatusStates) {
        self.modified_field = modified_field;
        self.previous_setting_field = std::time::Instant::now();
        if self.new_state != AutoStatusStates::Unknown
            && self.modified_field != AutoStatusFields::Unknown
            && self.previous_setting_state.elapsed() < AUTO_STATUS_WAIT_THRESHOLD
        {
            return self.return_new_status();
        } else {
            return (AutoStatusFields::Unknown, AutoStatusStates::Unknown);
        }
    }
}

struct PassiveTimer {
    enabled: bool,
    passive_counter: u32,
}

impl PassiveTimer {
    pub fn new() -> PassiveTimer {
        Self {
            enabled: false,
            passive_counter: 60,
        }
    }

    pub fn tick(&mut self) {
        if self.enabled && self.passive_counter != 0 {
            self.passive_counter -= 1;

            println!("{}", self.passive_counter);
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.passive_counter = 60;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

enum InputData {
    UartData(UartData),
    PinsData(PinsData),
    IrCommand(IrFrame),
}

#[derive(Debug)]
struct UartData {
    yellow_red: bool,
    white_red: bool,
    red: bool,

    yellow_green: bool,
    white_green: bool,
    green: bool,

    apparel_sound: bool,

    symbol: bool,

    on_timer: bool,

    minutes: u32,
    dec_seconds: u32,
    seconds: u32,

    timer_sound: bool,
    score_left: u32,
    score_right: u32,
    period: u32,

    yellow_card_left: bool,
    red_card_left: bool,
    yellow_card_right: bool,
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

fn uart_handler(tx: mpsc::Sender<InputData>) {
    let mut port = serial::open("/dev/ttyS2").unwrap();

    let settings = serial::PortSettings {
        baud_rate: serial::BaudRate::Baud38400,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };

    port.configure(&settings).unwrap();
    port.set_timeout(Duration::from_secs(60)).unwrap();

    let mut buf: [u8; 8] = [0; 8];
    let mut ind: usize = 0;

    for byte in port.bytes() {
        match byte {
            Err(_) => {}
            Ok(byte_val) => {
                println!("Got byte {:#010b}", byte_val);
                if byte_val >> 5 == 0 {
                    ind = 0;
                }

                if byte_val >> 5 == ind as u8 {
                    buf[ind] = byte_val;
                    ind += 1;

                    if ind == 8 {
                        ind = 0;

                        println!("Sending data from uart_handler");

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
    RightPenalty,

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
            11 => IrCommands::RightPenalty,

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

fn rc5_reciever(tx: mpsc::Sender<InputData>) {
    let line = crate::gpio::get_pin_by_phys_number(3).unwrap();
    let mut chip = gpio_cdev::Chip::new(format!("/dev/gpiochip{}", line.chip)).unwrap();

    let mut last_interrupt_time: u64 = 0u64;

    let mut recieve_buf: [i32; 28] = [0; 28];
    let mut index = 0;

    let mut last_toggle_value = -1;

    for event in chip
        .get_line(line.line)
        .unwrap()
        .events(
            gpio_cdev::LineRequestFlags::INPUT,
            gpio_cdev::EventRequestFlags::BOTH_EDGES,
            "gpioevents",
        )
        .unwrap()
    {
        let event = event.unwrap();

        let val = match event.event_type() {
            gpio_cdev::EventType::RisingEdge => 0,
            gpio_cdev::EventType::FallingEdge => 1,
        };
        let mut count = 0;

        if event.timestamp() - last_interrupt_time > 889 * 1000 * 5 / 2 {
            recieve_buf[0] = val;
            index = 1;
            count = 0;
        } else if event.timestamp() - last_interrupt_time > 889 * 1000 * 3 / 2 {
            count = 2;
        } else if event.timestamp() - last_interrupt_time > 889 * 1000 * 1 / 2 {
            count = 1;
        }

        for _ in 0..count {
            recieve_buf[index] = val;
            index += 1;

            if index == 27 {
                recieve_buf[index] = 1 - val;
                index += 1;
            }

            if index == 28 {
                for i in 0..14 {
                    if recieve_buf[i * 2] + recieve_buf[i * 2 + 1] != 1 {
                        println!("Bad buffer");
                        index = 0;
                        break;
                    }
                }
            }

            if index == 28 {
                let rc5_frame: Vec<i32> = recieve_buf.iter().step_by(2).cloned().collect();

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

fn pins_handler(tx: mpsc::Sender<InputData>) {
    let mut chips = Vec::<gpio_cdev::Chip>::new();

    for path in &["/dev/gpiochip0", "/dev/gpiochip1"] {
        if let Ok(chip) = gpio_cdev::Chip::new(path) {
            chips.push(chip);
        } else {
            println!("Failed to open chip {}", path);
        }
    }

    let gpio_pin_wireless = crate::gpio::get_pin_by_phys_number(7).unwrap();
    let gpio_line_wireless = chips[gpio_pin_wireless.chip as usize]
        .get_line(gpio_pin_wireless.line)
        .unwrap();
    let gpio_handle_wireless = gpio_line_wireless
        .request(gpio_cdev::LineRequestFlags::INPUT, 0, "read-input")
        .unwrap();

    let gpio_pin_weapon_0 = crate::gpio::get_pin_by_phys_number(32).unwrap();
    let gpio_line_weapon_0 = chips[gpio_pin_weapon_0.chip as usize]
        .get_line(gpio_pin_weapon_0.line)
        .unwrap();
    let gpio_handle_weapon_0 = gpio_line_weapon_0
        .request(gpio_cdev::LineRequestFlags::INPUT, 0, "read-input")
        .unwrap();
    let gpio_pin_weapon_1 = crate::gpio::get_pin_by_phys_number(36).unwrap();
    let gpio_line_weapon_1 = chips[gpio_pin_weapon_1.chip as usize]
        .get_line(gpio_pin_weapon_1.line)
        .unwrap();
    let gpio_handle_weapon_1 = gpio_line_weapon_1
        .request(gpio_cdev::LineRequestFlags::INPUT, 0, "read-input")
        .unwrap();

    let gpio_pin_weapon_btn = crate::gpio::get_pin_by_phys_number(37).unwrap();
    let gpio_line_weapon_btn = chips[gpio_pin_weapon_btn.chip as usize]
        .get_line(gpio_pin_weapon_btn.line)
        .unwrap();
    let gpio_handle_weapon_btn = gpio_line_weapon_btn
        .request(gpio_cdev::LineRequestFlags::INPUT, 0, "read-input")
        .unwrap();

    let mut old_pins_data = Option::None;

    loop {
        let new_pins_data = PinsData {
            weapon: gpio_handle_weapon_0.get_value().unwrap() * 2
                + gpio_handle_weapon_1.get_value().unwrap(),
            weapon_select_btn: gpio_handle_weapon_btn.get_value().unwrap() == 0u8,
        };

        if old_pins_data.as_ref() != Some(&new_pins_data) {
            tx.send(InputData::PinsData(new_pins_data.clone())).unwrap();
        }

        old_pins_data = Some(new_pins_data);

        thread::sleep(Duration::from_millis(10));
    }
}
