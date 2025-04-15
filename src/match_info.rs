#[derive(PartialEq, Clone, Copy)]
pub enum Priority {
    Left,
    None,
    Right,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Left => write!(f, "Left"),
            Priority::None => write!(f, "None"),
            Priority::Right => write!(f, "Right"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Weapon {
    Unknown,
    Epee,
    Sabre,
    Fleuret,
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weapon::Unknown => write!(f, "Unknown"),
            Weapon::Epee => write!(f, "Epee"),
            Weapon::Sabre => write!(f, "Sabre"),
            Weapon::Fleuret => write!(f, "Fleuret"),
        }
    }
}

pub enum ProgramState {
    Running,
    Exiting,
}

pub struct MatchInfo {
    pub modified_count: u32,

    pub weapon: Weapon,
    pub left_score: u32,
    pub right_score: u32,
    pub timer: u32,
    pub last_ten_seconds: bool,
    pub timer_running: bool,
    pub period: u32,
    pub priority: Priority,
    pub passive_indicator: u32,
    pub passive_counter: u32,

    pub auto_score_on: bool,
    pub auto_timer_on: bool,

    pub left_red_led_on: bool,
    pub left_white_led_on: bool,
    pub right_green_led_on: bool,
    pub right_white_led_on: bool,

    pub left_caution: bool,
    pub left_penalty: bool,
    pub right_caution: bool,
    pub right_penalty: bool,

    pub left_pcard_bot: bool,
    pub left_pcard_top: bool,
    pub right_pcard_bot: bool,
    pub right_pcard_top: bool,

    pub cyrano_online: bool,

    pub program_state: ProgramState,
}

impl MatchInfo {
    pub fn new() -> Self {
        Self {
            modified_count: 0,
            weapon: Weapon::Epee,
            left_score: 0,
            right_score: 0,
            timer: 300,
            last_ten_seconds: false,
            timer_running: false,
            period: 1,
            priority: Priority::None,
            passive_indicator: 0,
            passive_counter: 60,

            auto_score_on: false,
            auto_timer_on: false,

            left_red_led_on: false,
            left_white_led_on: false,
            right_green_led_on: false,
            right_white_led_on: false,

            left_caution: false,
            left_penalty: false,
            right_caution: false,
            right_penalty: false,

            left_pcard_bot: false,
            left_pcard_top: false,
            right_pcard_bot: false,
            right_pcard_top: false,

            cyrano_online: false,

            program_state: ProgramState::Running,
        }
    }
}
