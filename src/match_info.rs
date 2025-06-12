#[derive(PartialEq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CompetitionType {
    Individual,
    Team,
}

impl std::str::FromStr for CompetitionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "i" => Ok(Self::Individual),
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

#[derive(PartialEq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ProgramState {
    Running,
    Exiting,
}

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
pub struct FencerInfo {
    pub id: u32,
    pub name: String,   // 20
    pub nation: String, // 3
    pub score: u32,
    pub status: FencerStatus,
    pub yellow_card: u32,
    pub red_card: u32,
    pub color_light: bool,
    pub white_light: bool,
    pub medical_interventions: u32,
    pub reserve_introduction: bool,
    pub p_card: u32,
}

impl FencerInfo {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            nation: "".to_string(),
            score: 0,
            status: FencerStatus::Undefined,
            yellow_card: 0,
            red_card: 0,
            color_light: false,
            white_light: false,
            medical_interventions: 0,
            reserve_introduction: false,
            p_card: 0,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct MatchInfo {
    pub program_state: ProgramState,
    pub modified_count: u32,

    pub weapon: Weapon,
    pub timer: u32,
    pub last_ten_seconds: bool,
    pub timer_running: bool,
    pub period: u32,
    pub priority: Priority,
    pub passive_indicator: u32,
    pub passive_counter: u32,

    pub auto_score_on: bool,
    pub auto_timer_on: bool,

    pub cyrano_online: bool,

    pub piste: String,
    pub competition_id: String,
    pub phase: u32,
    pub poul_tab: String,
    pub match_number: u32,
    pub round_number: u32,
    pub time: String,
    pub stopwatch: String,
    pub competition_type: Option<CompetitionType>,

    pub referee: RefereeInfo,
    pub left_fencer: FencerInfo,
    pub right_fencer: FencerInfo,
}

impl MatchInfo {
    pub fn new() -> Self {
        Self {
            program_state: ProgramState::Running,
            modified_count: 0,

            weapon: Weapon::Epee,
            // left_score: 0,
            // right_score: 0,
            timer: 300,
            last_ten_seconds: false,
            timer_running: false,
            period: 1,
            priority: Priority::None,
            passive_indicator: 0,
            passive_counter: 60,

            auto_score_on: false,
            auto_timer_on: false,

            cyrano_online: false,

            piste: "".to_string(),
            competition_id: "".to_string(),
            phase: 0,
            poul_tab: "".to_string(),
            match_number: 0,
            round_number: 0,

            time: "".to_string(),
            stopwatch: "".to_string(),
            competition_type: None,

            referee: RefereeInfo::new(),
            left_fencer: FencerInfo::new(),
            right_fencer: FencerInfo::new(),
        }
    }
}
