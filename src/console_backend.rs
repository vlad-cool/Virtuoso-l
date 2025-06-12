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

    LeftCaution,
    LeftPenalty,
    RightCaution,
    RightPenalty,

    LeftPCard,
    RightPCard,

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

            Field::LeftCaution => write!(f, "Left Caution"),
            Field::LeftPenalty => write!(f, "Left Penalty"),
            Field::RightCaution => write!(f, "Right Caution"),
            Field::RightPenalty => write!(f, "Right Penalty"),

            Field::LeftPCard => write!(f, "Left PCard"),
            Field::RightPCard => write!(f, "Right PCard"),

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

        "leftcaution" => Field::LeftCaution,
        "leftpenalty" => Field::LeftPenalty,
        "rightcaution" => Field::RightCaution,
        "rightpenalty" => Field::RightPenalty,

        "leftpcard" => Field::LeftPCard,
        "rightpcard" => Field::RightPCard,

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
            Field::Time => match_info_data.timer = value,
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

            Field::LeftCaution => match_info_data.left_fencer.yellow_card = value,
            Field::LeftPenalty => match_info_data.left_fencer.red_card = value,
            Field::RightCaution => match_info_data.right_fencer.yellow_card = value,
            Field::RightPenalty => match_info_data.right_fencer.red_card = value,

            Field::LeftPCard => match_info_data.left_fencer.p_card = value,
            Field::RightPCard => match_info_data.right_fencer.p_card = value,

            Field::AutoScore => match_info_data.auto_score_on = value > 0,
            Field::AutoTimer => match_info_data.auto_timer_on = value > 0,

            Field::PassiveCounter => match_info_data.passive_counter = value,
            Field::PassiveIndicator => match_info_data.passive_indicator = value,

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
            Field::Time => println!("{}", match_info_data.timer),
            Field::LastTenSeconds => println!("{}", match_info_data.last_ten_seconds),
            Field::TimerRunning => println!("{}", match_info_data.timer_running),
            Field::Period => println!("{}", match_info_data.period),

            Field::Weapon => println!("{}", match_info_data.weapon),

            Field::Priority => println!("{}", match_info_data.priority),

            Field::LeftColorLed => println!("{}", match_info_data.left_fencer.color_light),
            Field::LeftWhiteLed => println!("{}", match_info_data.left_fencer.white_light),
            Field::RightColorLed => println!("{}", match_info_data.right_fencer.color_light),
            Field::RightWhiteLed => println!("{}", match_info_data.right_fencer.white_light),

            Field::LeftCaution => println!("{}", match_info_data.left_fencer.yellow_card),
            Field::LeftPenalty => println!("{}", match_info_data.left_fencer.red_card),
            Field::RightCaution => println!("{}", match_info_data.right_fencer.yellow_card),
            Field::RightPenalty => println!("{}", match_info_data.right_fencer.red_card),

            Field::LeftPCard => println!("{}", match_info_data.left_fencer.p_card),
            Field::RightPCard => println!("{}", match_info_data.right_fencer.p_card),

            Field::AutoScore => println!("{}", match_info_data.auto_score_on),
            Field::AutoTimer => println!("{}", match_info_data.auto_timer_on),

            Field::PassiveCounter => println!("{}", match_info_data.passive_counter),
            Field::PassiveIndicator => println!("{}", match_info_data.passive_indicator),

            Field::Unknown => println!("Unknown field"),
        }
    }
}

impl modules::VirtuosoModule for ConsoleBackend {
    fn run(&mut self) {
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
