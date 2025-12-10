use std::io;
use std::time::Duration;

use crate::match_info::{self, ProgramState};
use crate::modules::CyranoCommand;
use crate::modules::{self, VirtuosoModuleContext};
use crate::virtuoso_logger::LoggerUnwrap;

pub struct ConsoleBackend {
    context: modules::VirtuosoModuleContext,
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
    PassiveTimerRunning,

    SettingsMenu,
    SettingsNextMenu,
    SettingsNextElement,
    SettingsPress,

    LeftFlag,

    Medical,

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
            Field::PassiveTimerRunning => write!(f, "Passive timer running"),

            Field::SettingsMenu => write!(f, "Settings Menu"),
            Field::SettingsNextMenu => write!(f, "Settings Next Menu"),
            Field::SettingsNextElement => write!(f, "Settings Next Element"),
            Field::SettingsPress => write!(f, "Settings Press"),

            Field::LeftFlag => write!(f, "Left Flag"),

            Field::Medical => write!(f, "Medical"),

            Field::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
enum Command {
    Set(Field, u32),
    Get(Field),
    Show,
    CyranoCommand(CyranoCommand),
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
        "passivetimerrunning" => Field::PassiveTimerRunning,

        "autoscore" => Field::AutoScore,
        "autotimer" => Field::AutoTimer,

        "settingsmenu" => Field::SettingsMenu,
        "menunext" => Field::SettingsNextMenu,
        "elementnext" => Field::SettingsNextElement,
        "menupress" => Field::SettingsPress,

        "leftflag" => Field::LeftFlag,

        "medical" => Field::Medical,

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
        ["cyrano", command] => match *command {
            "prev" => Command::CyranoCommand(CyranoCommand::CyranoPrev),
            "next" => Command::CyranoCommand(CyranoCommand::CyranoNext),
            "begin" => Command::CyranoCommand(CyranoCommand::CyranoBegin),
            "end" => Command::CyranoCommand(CyranoCommand::CyranoEnd),
            _ => Command::Unknown,
        },

        _ => Command::Unknown,
    }
}

impl ConsoleBackend {
    pub fn new(context: VirtuosoModuleContext) -> Self {
        Self { context }
    }

    fn set_field(&mut self, field: Field, value: u32) {
        let mut match_info_data = self.context.match_info.lock().unwrap();

        match field {
            Field::LeftScore => match_info_data.left_fencer.score = value,
            Field::RightScore => match_info_data.right_fencer.score = value,
            Field::Time => {
                let time: Duration = Duration::from_secs(value.into());

                match_info_data.timer_controller.sync(time, false);
            }
            Field::LastTenSeconds => match_info_data.last_ten_seconds = value > 0,
            Field::TimerRunning => match_info_data.timer_controller.start_stop(value > 0),
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

            Field::PassiveCounter => println!("Setting passive counter is not implemented yet"),
            Field::PassiveIndicator => {
                println!("Setting passive indicator is not implemented yet")
            }
            Field::PassiveTimerRunning => {
                if value > 0 {
                    // match_info_data.passive_timer.enable();
                } else {
                    // match_info_data.passive_timer.disable();
                }
            }

            Field::SettingsMenu => {
                self.context
                    .settings_menu_shown
                    .store(value > 0, std::sync::atomic::Ordering::Relaxed);
            }
            Field::SettingsNextMenu => {
                let mut menu: std::sync::MutexGuard<'_, modules::SettingsMenu> =
                    self.context.settings_menu.lock().unwrap();
                menu.next();
            }
            Field::SettingsNextElement => {
                let mut menu: std::sync::MutexGuard<'_, modules::SettingsMenu> =
                    self.context.settings_menu.lock().unwrap();
                menu.get_item_mut().next();
            }
            Field::SettingsPress => {
                let mut menu: std::sync::MutexGuard<'_, modules::SettingsMenu> =
                    self.context.settings_menu.lock().unwrap();
                menu.get_item_mut().get_active_mut().press(
                    &self.context.logger,
                    self.context.settings_menu_shown.clone(),
                );
            }

            Field::Medical => {
                if value == 0 {
                    match_info_data.timer_controller.stop_medical_emergency();
                } else {
                    match_info_data
                        .timer_controller
                        .start_medical_emergency(if value % 2 == 0 {
                            match_info::Side::Left
                        } else {
                            match_info::Side::Right
                        });
                }
            }

            Field::LeftFlag => match_info_data.left_fencer.nation = "RUS".into(),

            Field::Unknown => {
                return;
            }
        }

        std::mem::drop(match_info_data);
        self.context.match_info_data_updated();
    }

    fn print_field(&self, field: Field) {
        let match_info_data = self.context.match_info.lock().unwrap();

        match field {
            Field::LeftScore => println!("{}", match_info_data.left_fencer.score),
            Field::RightScore => println!("{}", match_info_data.right_fencer.score),
            Field::Time => println!(
                "{}",
                match_info_data.timer_controller.get_main_time_string().0
            ),
            Field::LastTenSeconds => println!("{}", match_info_data.last_ten_seconds),
            Field::TimerRunning => {
                println!("{}", match_info_data.timer_controller.is_timer_running())
            }
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
                println!(
                    "not final {}",
                    match_info_data.timer_controller.get_passive_counter()
                )
            }
            Field::PassiveIndicator => {
                println!(
                    "not final {}",
                    match_info_data.timer_controller.get_passive_counter()
                )
            }
            Field::PassiveTimerRunning => {}

            Field::SettingsMenu => {}
            Field::SettingsNextMenu => {}
            Field::SettingsNextElement => {}
            Field::SettingsPress => {}

            Field::Medical => {}

            Field::LeftFlag => {}

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

            if self.context.match_info.lock().unwrap().program_state == ProgramState::Exiting {
                break;
            }

            if input == "" {
                continue;
            }

            let command = parse_command(&input);

            match command {
                Command::Set(field, value) => self.set_field(field, value),
                Command::Get(field) => self.print_field(field),
                Command::Show => println!("{:?}", self.context.match_info.lock().unwrap()),
                Command::Unknown => println!("Unknown command or invalid format"),
                Command::CyranoCommand(command) => {
                    self.context
                        .cyrano_command_tx
                        .send(command)
                        .log_err(&self.context.logger);
                }
            }
        }
    }
}
