use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::{Duration, Instant};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum WarningCard {
    None,
    Yellow(u8),
    Red(u8),
    Black(u8),
}

#[allow(dead_code)]
impl WarningCard {
    const NUM_YELLOW: u8 = 1;
    const NUM_RED: u8 = 15;
    const NUM_BLACK: u8 = 1;

    pub fn inc(&mut self) {
        *self = match self {
            Self::None => Self::Yellow(1),
            Self::Yellow(Self::NUM_YELLOW) => Self::Red(1),
            Self::Red(Self::NUM_RED) => Self::Black(1),
            Self::Black(Self::NUM_BLACK) => Self::None,
            Self::Yellow(n) => Self::Yellow(*n + 1),
            Self::Red(n) => Self::Red(*n + 1),
            Self::Black(n) => Self::Black(*n + 1),
        };
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Yellow(n) => (1 + *n).into(),
            Self::Red(n) => (1 + Self::NUM_YELLOW + *n).into(),
            Self::Black(n) => (1 + Self::NUM_YELLOW + Self::NUM_RED + *n).into(),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Yellow(n) => (1 + *n).into(),
            Self::Red(n) => (1 + Self::NUM_YELLOW + *n).into(),
            Self::Black(n) => (1 + Self::NUM_YELLOW + Self::NUM_RED + *n).into(),
        }
    }

    pub fn has_yellow(&self) -> u32 {
        match self {
            Self::None => 0,
            _ => 1,
        }
    }

    pub fn num_red(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Yellow(_) => 0,
            _ => self.to_u32() - 1 - Self::NUM_YELLOW as u32,
        }
    }
}

impl std::fmt::Display for WarningCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Yellow(1) => write!(f, "Yellow"),
            Self::Yellow(n) => write!(f, "Yellow x {n}"),
            Self::Red(1) => write!(f, "Red"),
            Self::Red(n) => write!(f, "Red x {n}"),
            Self::Black(1) => write!(f, "Black"),
            Self::Black(n) => write!(f, "Black x {n}"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PassiveCard {
    None,
    Yellow(u8),
    Red(u8),
    Black(u8),
}

#[allow(dead_code)]
impl PassiveCard {
    const NUM_YELLOW: u8 = 1;
    const NUM_RED: u8 = 2;
    const NUM_BLACK: u8 = 2;

    pub fn inc(&mut self) {
        *self = match self {
            Self::None => Self::Yellow(1),
            Self::Yellow(Self::NUM_YELLOW) => Self::Red(1),
            Self::Red(Self::NUM_RED) => Self::Black(1),
            Self::Black(Self::NUM_BLACK) => Self::None,
            Self::Yellow(n) => Self::Yellow(*n + 1),
            Self::Red(n) => Self::Red(*n + 1),
            Self::Black(n) => Self::Black(*n + 1),
        };
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Yellow(n) => (1 + *n).into(),
            Self::Red(n) => (1 + Self::NUM_YELLOW + *n).into(),
            Self::Black(n) => (1 + Self::NUM_YELLOW + Self::NUM_RED + *n).into(),
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Yellow(n) => (1 + *n).into(),
            Self::Red(n) => (1 + Self::NUM_YELLOW + *n).into(),
            Self::Black(n) => (1 + Self::NUM_YELLOW + Self::NUM_RED + *n).into(),
        }
    }
}

impl std::fmt::Display for PassiveCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Yellow(1) => write!(f, "Yellow"),
            Self::Yellow(n) => write!(f, "Yellow x {n}"),
            Self::Red(1) => write!(f, "Red"),
            Self::Red(n) => write!(f, "Red x {n}"),
            Self::Black(1) => write!(f, "Black"),
            Self::Black(n) => write!(f, "Black x {n}"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Priority {
    Left,
    None,
    Right,
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "l" => Ok(Self::Left),
            "left" => Ok(Self::Left),
            "n" => Ok(Self::None),
            "none" => Ok(Self::None),
            "r" => Ok(Self::Right),
            "right" => Ok(Self::Right),
            _ => Err(format!("Unknown weapon type: {}", s)),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.width() == Some(1) {
            match self {
                Self::Left => write!(f, "L"),
                Self::None => write!(f, "N"),
                Self::Right => write!(f, "R"),
            }
        } else {
            match self {
                Self::Left => write!(f, "Left"),
                Self::None => write!(f, "None"),
                Self::Right => write!(f, "Right"),
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Weapon {
    Epee,
    Sabre,
    Fleuret,
}

impl std::str::FromStr for Weapon {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "f" => Ok(Self::Fleuret),
            "fleuret" => Ok(Self::Fleuret),
            "e" => Ok(Self::Epee),
            "epee" => Ok(Self::Epee),
            "s" => Ok(Self::Sabre),
            "sabre" => Ok(Self::Sabre),
            _ => Err(format!("Unknown weapon type: {}", s)),
        }
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.width() == Some(1) {
            match self {
                Weapon::Epee => write!(f, "E"),
                Weapon::Sabre => write!(f, "S"),
                Weapon::Fleuret => write!(f, "F"),
            }
        } else {
            match self {
                Weapon::Epee => write!(f, "Epee"),
                Weapon::Sabre => write!(f, "Sabre"),
                Weapon::Fleuret => write!(f, "Fleuret"),
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CompetitionType {
    Individual,
    Team,
}

impl std::str::FromStr for CompetitionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "i" => Ok(Self::Individual),
            "1" => Ok(Self::Individual), // WTF? Not as in documentation, but in test software
            "individual" => Ok(Self::Individual),
            "t" => Ok(Self::Team),
            "team" => Ok(Self::Team),
            _ => Err(format!("Unknown competition type: {}", s)),
        }
    }
}

impl std::fmt::Display for CompetitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.width() == Some(1) {
            match self {
                CompetitionType::Individual => write!(f, "I"),
                CompetitionType::Team => write!(f, "T"),
            }
        } else {
            match self {
                CompetitionType::Individual => write!(f, "Individual"),
                CompetitionType::Team => write!(f, "Team"),
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FencerStatus {
    Undefined,
    Victorie,
    Defaite,
    Abandonment,
    Exclusion,
}

impl std::str::FromStr for FencerStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "u" => Ok(Self::Undefined),
            "undefined" => Ok(Self::Undefined),
            "v" => Ok(Self::Victorie),
            "victorie" => Ok(Self::Victorie),
            "d" => Ok(Self::Defaite),
            "defaite" => Ok(Self::Defaite),
            "a" => Ok(Self::Abandonment),
            "abandonment" => Ok(Self::Abandonment),
            "e" => Ok(Self::Exclusion),
            "exclusion" => Ok(Self::Exclusion),
            "n" => Ok(Self::Undefined),
            _ => Err(format!("Unknown fencer status: {}", s)),
        }
    }
}

impl std::fmt::Display for FencerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.width() == Some(1) {
            match self {
                FencerStatus::Undefined => write!(f, "U"),
                FencerStatus::Victorie => write!(f, "V"),
                FencerStatus::Defaite => write!(f, "D"),
                FencerStatus::Abandonment => write!(f, "A"),
                FencerStatus::Exclusion => write!(f, "E"),
            }
        } else {
            match self {
                FencerStatus::Undefined => write!(f, "Undefined"),
                FencerStatus::Victorie => write!(f, "Victorie"),
                FencerStatus::Defaite => write!(f, "Defaite"),
                FencerStatus::Abandonment => write!(f, "Abandonment"),
                FencerStatus::Exclusion => write!(f, "Exclusion"),
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PassiveTimer {
    enabled: bool,
    passive_counter: u32,
    #[serde(
        serialize_with = "serialize_instant_as_elapsed",
        deserialize_with = "deserialize_duration_to_instant"
    )]
    last_updated: Instant,
}

#[allow(dead_code)]
impl PassiveTimer {
    pub fn new() -> PassiveTimer {
        Self {
            enabled: false,
            passive_counter: 60,
            last_updated: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        if self.enabled && self.passive_counter != 0 {
            self.passive_counter -= 1;
            self.last_updated = Instant::now();
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.passive_counter = 60;
        self.last_updated = Instant::now();
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.last_updated = Instant::now();
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.last_updated = Instant::now();
    }

    pub fn get_counter(&self) -> u32 {
        self.passive_counter
    }

    pub fn get_indicator(&self) -> u32 {
        let res: u32 = if self.enabled {
            ((60 - self.passive_counter) * 1000 + self.last_updated.elapsed().as_millis() as u32)
                / 50
        } else {
            (60 - self.passive_counter) * 1000 / 50
        };
        if res > 1000 { 1000 } else { res }
    }

    /// Returns true if timer has minimal or maximal value
    pub fn on_edge(&self) -> bool {
        self.get_counter() == 0 || self.get_counter() == 60
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TimerController {
    last_second: bool,
    time: u32,
    timer_running: bool,
    prev_second_value: u32,
    second_changed: bool,
    #[serde(
        serialize_with = "serialize_instant_as_elapsed",
        deserialize_with = "deserialize_duration_to_instant"
    )]
    last_updated: Instant,
    last_updated_freezed: u32,
}

impl PartialEq for TimerController {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.timer_running == other.timer_running
    }
}

#[allow(dead_code)]
impl TimerController {
    pub fn new() -> Self {
        Self {
            last_second: false,
            time: 3 * 60 * 1000,
            timer_running: false,
            prev_second_value: 0,
            second_changed: false,
            last_updated: Instant::now(),
            last_updated_freezed: 0,
        }
    }

    pub fn set_time(&mut self, timer_m: u32, timer_d: u32, timer_s: u32) {
        self.last_updated = Instant::now();

        if timer_d >= 6 {
            self.last_second = true;
        }
        if timer_m > 0 {
            self.last_second = false;
        }

        if self.last_second {
            if timer_m != self.prev_second_value {
                self.second_changed = true;
            } else {
                self.second_changed = false;
            }
            self.prev_second_value = timer_m;

            self.time = timer_d * 100 + timer_s * 10;
        } else {
            if timer_s != self.prev_second_value {
                self.second_changed = true;
            } else {
                self.second_changed = false;
            }
            self.prev_second_value = timer_s;

            self.time = (timer_m * 60 + timer_d * 10 + timer_s) * 1000;
        }
    }

    pub fn stop_timer(&mut self) {
        self.last_updated_freezed = self.last_updated.elapsed().as_millis() as u32;
        self.timer_running = false;
    }

    pub fn start_timer(&mut self) {
        self.timer_running = true;
        self.last_updated = Instant::now();
    }

    pub fn get_millis(&self) -> u32 {
        if self.timer_running {
            if self.time > self.last_updated.elapsed().as_millis() as u32 {
                let time: u32 = self.time - self.last_updated.elapsed().as_millis() as u32;

                if (time / 1000 + 1) % 10 != self.prev_second_value {
                    time - time % 1000
                } else {
                    time
                }
                // time
            } else {
                0
            }
        } else {
            if self.time > self.last_updated_freezed {
                self.time - self.last_updated_freezed
            } else {
                0
            }
        }
    }

    pub fn get_second_changed(&self) -> bool {
        self.second_changed
    }

    pub fn get_time(&self) -> Duration {
        Duration::from_millis(self.get_millis() as u64 + 999)
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ProgramState {
    Running,
    Exiting,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct RefereeInfo {
    pub id: u32,
    pub name: String,
    pub nation: String,
}

impl RefereeInfo {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            nation: "".to_string(),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct FencerInfo {
    pub id: u32,
    pub name: String,   // 20
    pub nation: String, // 3
    pub score: u32,
    pub status: FencerStatus,
    pub color_light: bool,
    pub white_light: bool,
    pub medical_interventions: u32,
    pub reserve_introduction: bool,

    pub warning_card: WarningCard,
    pub passive_card: PassiveCard,
}

impl FencerInfo {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            nation: "".to_string(),
            score: 0,
            status: FencerStatus::Undefined,
            color_light: false,
            white_light: false,
            medical_interventions: 0,
            reserve_introduction: false,
            warning_card: WarningCard::None,
            passive_card: PassiveCard::None,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MatchInfo {
    pub program_state: ProgramState,

    pub weapon: Weapon,
    // pub timer: u32,
    pub last_ten_seconds: bool,
    pub timer_running: bool,
    pub period: u32,
    pub priority: Priority,
    #[serde(
        serialize_with = "serialize_instant_as_elapsed",
        deserialize_with = "deserialize_duration_to_instant"
    )]
    pub priority_updated: Instant,
    pub auto_score_on: bool,
    pub auto_timer_on: bool,

    pub cyrano_online: bool,
    pub cyrano_active: bool,

    pub piste: String,
    pub competition_id: String,
    pub phase: u32,
    pub poul_tab: String,
    pub match_number: u32,
    pub round_number: u32,
    pub time: String,
    pub display_message: String,
    #[serde(
        serialize_with = "serialize_instant_as_elapsed",
        deserialize_with = "deserialize_duration_to_instant"
    )]
    pub display_message_updated: Instant,
    // pub stopwatch: String,
    pub competition_type: Option<CompetitionType>,

    pub timer_controller: TimerController,
    pub passive_timer: PassiveTimer,

    pub referee: RefereeInfo,
    pub left_fencer: FencerInfo,
    pub right_fencer: FencerInfo,
}

impl MatchInfo {
    pub fn new() -> Self {
        Self {
            program_state: ProgramState::Running,

            weapon: Weapon::Epee,
            // left_score: 0,
            // right_score: 0,
            // timer: 300,
            last_ten_seconds: false,
            timer_running: false,
            period: 1,
            priority: Priority::None,
            priority_updated: Instant::now(),
            // passive_indicator: 0,
            // passive_counter: 60,
            auto_score_on: false,
            auto_timer_on: false,

            cyrano_online: false,
            cyrano_active: false,

            piste: "".to_string(),
            competition_id: "".to_string(),
            phase: 0,
            poul_tab: "".to_string(),
            match_number: 0,
            round_number: 0,

            time: "".to_string(),
            display_message: "".to_string(),
            display_message_updated: Instant::now(),
            // stopwatch: "".to_string(),
            competition_type: None,

            timer_controller: TimerController::new(),
            passive_timer: PassiveTimer::new(),

            referee: RefereeInfo::new(),
            left_fencer: FencerInfo::new(),
            right_fencer: FencerInfo::new(),
        }
    }
}

fn serialize_instant_as_elapsed<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let duration: Duration = instant.elapsed();
    duration.serialize(serializer)
}

fn deserialize_duration_to_instant<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: Deserializer<'de>,
{
    let duration: Duration = Duration::deserialize(deserializer)?;
    Ok(Instant::now() - duration)
}
