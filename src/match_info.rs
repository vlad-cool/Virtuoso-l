use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::{Duration, Instant};

#[cfg(feature = "cyrano_server")]
use crate::cyrano_server::State;

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum WarningCard {
    None,
    Yellow(u8),
    Red(u8),
    Black(u8),
}

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

// #[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
// pub struct PassiveTimer {
//     enabled: bool,
//     passive_counter: u32,
//     #[serde(
//         serialize_with = "serialize_instant_as_elapsed",
//         deserialize_with = "deserialize_duration_to_instant"
//     )]
//     last_updated: Instant,
// }

// #[allow(dead_code)]
// impl PassiveTimer {
//     pub fn new() -> PassiveTimer {
//         Self {
//             enabled: false,
//             passive_counter: 60,
//             last_updated: Instant::now(),
//         }
//     }

//     pub fn tick(&mut self) {
//         if self.enabled && self.passive_counter != 0 {
//             self.passive_counter -= 1;
//             self.last_updated = Instant::now();
//         }
//     }

//     pub fn reset(&mut self) {
//         self.enabled = false;
//         self.passive_counter = 60;
//         self.last_updated = Instant::now();
//     }

//     pub fn enable(&mut self) {
//         self.enabled = true;
//         self.last_updated = Instant::now();
//     }

//     pub fn disable(&mut self) {
//         self.enabled = false;
//         self.last_updated = Instant::now();
//     }

//     pub fn get_counter(&self) -> u32 {
//         self.passive_counter
//     }

//     pub fn get_indicator(&self) -> u32 {
//         let res: u32 = if self.enabled {
//             ((60 - self.passive_counter) * 1000 + self.last_updated.elapsed().as_millis() as u32)
//                 / 50
//         } else {
//             (60 - self.passive_counter) * 1000 / 50
//         };
//         if res > 1000 { 1000 } else { res }
//     }

//     /// Returns true if timer has minimal or maximal value
//     pub fn on_edge(&self) -> bool {
//         self.get_counter() == 0 || self.get_counter() == 60
//     }
// }

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
// pub enum MainTimer<const MAX_SECONDS: u64> {
//     #[serde(
//         serialize_with = "serialize_instant_as_elapsed",
//         deserialize_with = "deserialize_duration_to_instant"
//     )]
//     Running(Instant),
//     Stopped(Duration),
// }

// impl<const MAX_SECONDS: u64> MainTimer<MAX_SECONDS> {
//     pub fn is_running(&self) -> bool {
//         match self {
//             Self::Running(_) => true,
//             Self::Stopped(_) => false,
//         }
//     }

//     pub fn start_timer(&mut self) {
//         if let Self::Stopped(time) = self {
//             *self = Self::Running(Instant::now() - *time)
//         }
//     }

//     pub fn stop_timer(&mut self) {
//         if let Self::Running(time) = self {
//             *self = Self::Stopped(time.elapsed())
//         }
//     }

//     pub fn set_timer_running(&mut self, new_state: bool) {
//         if new_state {
//             self.start_timer();
//         } else {
//             self.stop_timer();
//         }
//     }

//     pub fn set_time(&mut self, new_time: Duration) {
//         let new_time: Duration = Duration::from_secs(MAX_SECONDS)
//             .checked_sub(new_time)
//             .unwrap_or_default();
//         *self = if self.is_running() {
//             Self::Running(Instant::now() - new_time)
//         } else {
//             Self::Stopped(new_time)
//         };
//     }

//     pub fn get_time(&self) -> Duration {
//         let time: Duration = match self {
//             Self::Running(time) => time.elapsed(),
//             Self::Stopped(time) => *time,
//         };

//         Duration::from_secs(MAX_SECONDS)
//             .checked_sub(time)
//             .unwrap_or_default()
//     }

//     pub fn to_time_string(&self) -> String {
//         let time: Duration = self.get_time();

//         if time.as_secs() > 9 {
//             let minutes: u64 = time.as_secs() / 60;
//             let seconds: u64 = time.as_secs() % 60;
//             format!("{}:{:02}", minutes, seconds)
//         } else {
//             let seconds: u64 = time.as_secs();
//             let centiseconds: u32 = time.subsec_millis() / 10;
//             format!("{}.{:02}", seconds, centiseconds)
//         }
//     }
// }

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TimerController {
    #[serde(
        serialize_with = "serialize_optional_instant_as_elapsed",
        deserialize_with = "deserialize_optional_duration_to_instant"
    )]
    sync_time: Option<Instant>,

    main_timer: Duration,

    passive_timer_offset: Option<Duration>,
}

impl TimerController {
    fn get_sync_time(&self) -> Duration {
        if let Some(sync_time) = self.sync_time {
            sync_time.elapsed()
        } else {
            Duration::ZERO
        }
    }

    pub fn is_timer_running(&self) -> bool {
        match self.sync_time {
            Some(_) => true,
            None => false,
        }
    }

    pub fn new() -> Self {
        Self {
            sync_time: None,
            main_timer: Duration::from_secs(60 * 3),
            passive_timer_offset: Some(Duration::from_secs(60 * 2)),
        }
    }

    pub fn sync(&mut self, time: Duration, running: bool) {
        if running {
            self.sync_time = Some(Instant::now());
        } else {
            self.sync_time = None;
        }
        self.main_timer = time;
    }

    pub fn get_main_time(&self) -> Duration {
        self.main_timer.saturating_sub(self.get_sync_time())
    }

    pub fn get_main_time_string(&self) -> (String, Duration) {
        let time: Duration = self.get_main_time();

        (
            if time.as_secs() >= 10 {
                let time: u64 = (time + Duration::from_millis(999)).as_secs();

                let minutes: u64 = time / 60;
                let seconds: u64 = time % 60;
                format!("{}:{:02}", minutes, seconds)
            } else {
                let seconds: u64 = time.as_secs();
                let centiseconds: u32 = time.subsec_millis() / 10;
                format!("{}.{:02}", seconds, centiseconds)
            },
            time,
        )
    }

    pub fn reset_passive_timer(&mut self, active: bool) {
        self.passive_timer_offset = if active {
            self.get_main_time().checked_sub(Duration::from_secs(60))
        } else {
            None
        }
    }

    pub fn get_passive_timer(&self) -> Duration {
        if let Some(offset) = self.passive_timer_offset {
            self.get_main_time().saturating_sub(offset)
        } else {
            Duration::from_secs(60)
        }
    }

    pub fn get_passive_counter(&self) -> String {
        let time: Duration = self.get_passive_timer();

        format!("{:02}", time.as_secs_f32().ceil() as u32)
    }

    pub fn is_passive_timer_enabled(&self) -> bool {
        if let Some(_) = self.passive_timer_offset {
            true
        } else {
            false
        }
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
    pub period: u32,
    pub priority: Priority,
    #[serde(
        serialize_with = "serialize_instant_as_elapsed",
        deserialize_with = "deserialize_duration_to_instant"
    )]
    pub priority_updated: Instant,
    pub auto_score_on: bool,
    pub auto_timer_on: bool,

    #[cfg(feature = "cyrano_server")]
    pub cyrano_online: bool,
    #[cfg(feature = "cyrano_server")]
    pub cyrano_active: bool,
    #[cfg(feature = "cyrano_server")]
    pub cyrano_state: State,

    pub piste: String,
    pub competition_id: String,
    pub phase: u32,
    pub poul_tab: String,
    pub match_number: u32,
    pub round_number: u32,
    pub time: String,
    pub display_message: String,
    #[serde(
        serialize_with = "serialize_optional_instant_as_elapsed",
        deserialize_with = "deserialize_optional_duration_to_instant"
    )]
    pub display_message_updated: Option<Instant>,
    pub competition_type: Option<CompetitionType>,

    // pub main_timer: MainTimer<240>,
    // pub passive_timer: PassiveTimer,
    pub timer_controller: TimerController,

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
            period: 1,
            priority: Priority::None,
            priority_updated: Instant::now(),
            // passive_indicator: 0,
            // passive_counter: 60,
            auto_score_on: false,
            auto_timer_on: false,

#[cfg(feature = "cyrano_server")]
cyrano_online: false,
#[cfg(feature = "cyrano_server")]
cyrano_active: false,
#[cfg(feature = "cyrano_server")]
            cyrano_state: State::Waiting,

            piste: "".to_string(),
            competition_id: "".to_string(),
            phase: 0,
            poul_tab: "".to_string(),
            match_number: 0,
            round_number: 0,

            time: "".to_string(),
            display_message: "".to_string(),
            display_message_updated: None,
            // stopwatch: "".to_string(),
            competition_type: None,

            // main_timer: MainTimer::Stopped(Duration::from_secs(3 * 60)),
            // passive_timer: PassiveTimer::new(),
            timer_controller: TimerController::new(),

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

fn serialize_optional_instant_as_elapsed<S>(
    instant: &Option<Instant>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(instant) = instant {
        let duration: Duration = instant.elapsed();
        duration.serialize(serializer)
    } else {
        None::<Duration>.serialize(serializer)
    }
}

fn deserialize_optional_duration_to_instant<'de, D>(
    deserializer: D,
) -> Result<Option<Instant>, D::Error>
where
    D: Deserializer<'de>,
{
    let duration: Option<Duration> = Option::<Duration>::deserialize(deserializer)?;
    if let Some(duration) = duration {
        Ok(Some(Instant::now() - duration))
    } else {
        Ok(None)
    }
}
