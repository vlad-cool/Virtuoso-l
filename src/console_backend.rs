use std::io;
use std::sync::{Arc, Mutex};

use crate::match_info::{self, ProgramState};
use crate::modules;

pub struct ConsoleBackend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
}

#[derive(Debug)]
enum Field {
    LeftScore,
    RightScore,
    Time,
    LastTenSeconds,
    TimerRunning,
    Period,

    Weapon,

    Priority,

    LeftWhiteLed,
    LeftColorLed,
    RightWhiteLed,
    RightColorLed,

    LeftWarning,
    RightWarning,

    LeftPassive,
    RightPassive,

    AutoScore,
    AutoTimer,

    PassiveCounter,
    PassiveIndicator,

    Unknown,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::LeftScore => write!(f, "Left Score"),
            Field::RightScore => write!(f, "Right Score"),
            Field::Time => write!(f, "Time"),
            Field::LastTenSeconds => write!(f, "Last Ten Seconds"),
            Field::TimerRunning => write!(f, "Timer Running"),
            Field::Period => write!(f, "Period"),

            Field::Weapon => write!(f, "Weapon"),

            Field::Priority => write!(f, "Priority"),

            Field::LeftWhiteLed => write!(f, "Left White Led"),
            Field::LeftColorLed => write!(f, "Left Color Led"),
            Field::RightWhiteLed => write!(f, "Right White Led"),
            Field::RightColorLed => write!(f, "Right Color Led"),

            Field::LeftWarning => write!(f, "Left Caution"),
            Field::RightWarning => write!(f, "Right Caution"),

            Field::LeftPassive => write!(f, "Left PCard"),
            Field::RightPassive => write!(f, "Right PCard"),

            Field::AutoScore => write!(f, "Auto Score"),
            Field::AutoTimer => write!(f, "Auto Timer"),

            Field::PassiveCounter => write!(f, "Passive Counter"),
            Field::PassiveIndicator => write!(f, "Passive Indicator"),

            Field::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
enum Command {
    Set(Field, u32),
    Get(Field),
    Show,
    Unknown,
}

fn parse_field(input: &str) -> Field {
    match input {
        "leftscore" => Field::LeftScore,
        "rightscore" => Field::RightScore,
        "time" => Field::Time,
        "lasttenseconds" => Field::LastTenSeconds,
        "timerrunning" => Field::TimerRunning,
        "period" => Field::Period,

        "weapon" => Field::Weapon,

        "priority" => Field::Priority,

        "leftwhiteled" => Field::LeftWhiteLed,
        "leftcolorled" => Field::LeftColorLed,
        "rightwhiteled" => Field::RightWhiteLed,
        "rightcolorled" => Field::RightColorLed,

        "leftwarning" => Field::LeftWarning,
        "rightwarning" => Field::RightWarning,

        "leftpcard" => Field::LeftPassive,
        "rightpcard" => Field::RightPassive,

        "passivecounter" => Field::PassiveCounter,
        "passiveindicator" => Field::PassiveIndicator,

        "autoscore" => Field::AutoScore,
        "autotimer" => Field::AutoTimer,

        _ => Field::Unknown,
    }
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["set", variable, value] => match parse_field(&variable) {
            Field::Unknown => Command::Unknown,
            field => match value.parse::<u32>() {
                Ok(value) => Command::Set(field, value),
                _ => Command::Unknown,
            },
        },
        ["get", variable] => match parse_field(&variable) {
            Field::Unknown => Command::Unknown,
            field => Command::Get(field),
        },
        ["show"] => Command::Show,
        _ => Command::Unknown,
    }
}

impl ConsoleBackend {
    pub fn new(match_info: Arc<Mutex<match_info::MatchInfo>>) -> Self {
        Self { match_info }
    }

    fn set_field(&mut self, field: Field, value: u32) {
        let mut match_info_data = self.match_info.lock().unwrap();

        match field {
            Field::LeftScore => match_info_data.left_fencer.score = value,
            Field::RightScore => match_info_data.right_fencer.score = value,
            Field::Time => match_info_data
                .timer_controller
                .set_time(0, 0, value / 1000),
            Field::LastTenSeconds => match_info_data.last_ten_seconds = value > 0,
            Field::TimerRunning => match_info_data.timer_running = value > 0,
            Field::Period => match_info_data.period = value,

            Field::Weapon => {
                match_info_data.weapon = match value {
                    1 => match_info::Weapon::Epee,
                    2 => match_info::Weapon::Sabre,
                    3 => match_info::Weapon::Fleuret,
                    _ => match_info_data.weapon,
                }
            }

            Field::Priority => {
                match_info_data.priority = match value {
                    1 => match_info::Priority::Left,
                    2 => match_info::Priority::Right,
                    _ => match_info::Priority::None,
                }
            }

            Field::LeftColorLed => match_info_data.left_fencer.color_light = value > 0,
            Field::LeftWhiteLed => match_info_data.left_fencer.white_light = value > 0,
            Field::RightColorLed => match_info_data.right_fencer.color_light = value > 0,
            Field::RightWhiteLed => match_info_data.right_fencer.white_light = value > 0,

            // Field::LeftCaution => match_info_data.left_fencer.yellow_card = value,
            // Field::LeftPenalty => match_info_data.left_fencer.red_card = value,
            // Field::RightCaution => match_info_data.right_fencer.yellow_card = value,
            // Field::RightPenalty => match_info_data.right_fencer.red_card = value,
            Field::LeftWarning => println!("Not implemented yet"),
            Field::RightWarning => println!("Not implemented yet"),

            Field::LeftPassive => println!("Not implemented yet"),
            Field::RightPassive => println!("Not implemented yet"),

            Field::AutoScore => match_info_data.auto_score_on = value > 0,
            Field::AutoTimer => match_info_data.auto_timer_on = value > 0,

            Field::PassiveCounter => println!("Setting paassive counter is not implemented yet"),
            Field::PassiveIndicator => {
                println!("Setting paassive indicator is not implemented yet")
            }

            Field::Unknown => {
                println!("Unknown field");
                return;
            }
        }

        match_info_data.modified_count += 1;
    }

    fn print_field(&self, field: Field) {
        let match_info_data = self.match_info.lock().unwrap();

        match field {
            Field::LeftScore => println!("{}", match_info_data.left_fencer.score),
            Field::RightScore => println!("{}", match_info_data.right_fencer.score),
            Field::Time => println!("{}", match_info_data.timer_controller.get_millis() / 1000),
            Field::LastTenSeconds => println!("{}", match_info_data.last_ten_seconds),
            Field::TimerRunning => println!("{}", match_info_data.timer_running),
            Field::Period => println!("{}", match_info_data.period),

            Field::Weapon => println!("{}", match_info_data.weapon),

            Field::Priority => println!("{}", match_info_data.priority),

            Field::LeftColorLed => println!("{}", match_info_data.left_fencer.color_light),
            Field::LeftWhiteLed => println!("{}", match_info_data.left_fencer.white_light),
            Field::RightColorLed => println!("{}", match_info_data.right_fencer.color_light),
            Field::RightWhiteLed => println!("{}", match_info_data.right_fencer.white_light),

            Field::LeftWarning => println!("{}", match_info_data.left_fencer.warning_card),
            Field::RightWarning => println!("{}", match_info_data.right_fencer.warning_card),

            Field::LeftPassive => println!("{}", match_info_data.left_fencer.passive_card),
            Field::RightPassive => println!("{}", match_info_data.right_fencer.passive_card),

            Field::AutoScore => println!("{}", match_info_data.auto_score_on),
            Field::AutoTimer => println!("{}", match_info_data.auto_timer_on),

            Field::PassiveCounter => {
                println!("not final {}", match_info_data.passive_timer.get_counter())
            }
            Field::PassiveIndicator => {
                println!("not final {}", match_info_data.passive_timer.get_counter())
            }

            Field::Unknown => println!("Unknown field"),
        }
    }
}

impl modules::VirtuosoModule for ConsoleBackend {
    fn run(mut self) {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_ascii_lowercase();

            if self.match_info.lock().unwrap().program_state == ProgramState::Exiting {
                break;
            }

            if input == "" {
                continue;
            }

            let command = parse_command(&input);

            match command {
                Command::Set(field, value) => self.set_field(field, value),
                Command::Get(field) => self.print_field(field),
                Command::Show => println!("{:?}", self.match_info.lock().unwrap()),
                Command::Unknown => println!("Unknown command or invalid format"),
            }
        }
    }
}
