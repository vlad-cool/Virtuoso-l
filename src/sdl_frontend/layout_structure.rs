#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RectangleProperties {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub radius: u32,
}

impl RectangleProperties {
    pub const fn get_empty() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            radius: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TextProperties {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub font_size: u16,
}

impl TextProperties {
    pub const fn get_empty() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            font_size: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CyranoLayout {
    pub left_name: TextProperties,
    pub left_nation: TextProperties,
    pub left_flag: RectangleProperties,
    pub left_score: TextProperties,
    pub left_status: TextProperties,
    pub left_video: TextProperties,
    pub left_medical: TextProperties,
    pub left_reserve: TextProperties,
    pub left_id: TextProperties,
    pub left_photo: RectangleProperties,

    pub right_name: TextProperties,
    pub right_nation: TextProperties,
    pub right_flag: RectangleProperties,
    pub right_score: TextProperties,
    pub right_status: TextProperties,
    pub right_video: TextProperties,
    pub right_medical: TextProperties,
    pub right_reserve: TextProperties,
    pub right_id: TextProperties,
    pub right_photo: RectangleProperties,

    pub referee_name: TextProperties,
    pub referee_nation: TextProperties,

    pub piste: TextProperties,
    pub competition_phase: TextProperties,
    pub competition_type: TextProperties,
    pub poule_tableau_id: TextProperties,

    pub start_time: TextProperties,
    pub local_time: TextProperties,

    pub state: TextProperties,
    pub status: TextProperties,

    pub static_piste: TextProperties,
    pub static_competition_type: TextProperties,
    pub static_competition_phase: TextProperties,
    pub static_poule_tableau_id: TextProperties,
    pub static_start_time: TextProperties,
    pub static_referee: TextProperties,
}

pub struct Layout {
    pub background: RectangleProperties,
    // pub left_nation: TextProperties,
    // pub left_flag: RectangleProperties,
    // pub left_name: TextProperties,
    // pub right_nation: TextProperties,
    // pub right_flag: RectangleProperties,
    // pub right_name: TextProperties,
    pub score_l_l: TextProperties,
    pub score_l_r: TextProperties,
    pub score_r_l: TextProperties,
    pub score_r_r: TextProperties,
    pub epee: TextProperties,
    pub sabre: TextProperties,
    pub fleuret: TextProperties,
    pub disable_inactive_weapon: bool,
    pub auto_score_status: TextProperties,
    pub auto_timer_status: TextProperties,
    pub auto_status_only: bool,
    pub priority_l_cap: TextProperties,
    pub priority_l_text: TextProperties,
    pub priority_r_cap: TextProperties,
    pub priority_r_text: TextProperties,
    pub caution_l_rect: RectangleProperties,
    pub caution_l_text: TextProperties,
    pub passive_l_rect: RectangleProperties,
    pub passive_l_text: TextProperties,
    pub caution_r_rect: RectangleProperties,
    pub caution_r_text: TextProperties,
    pub passive_r_rect: RectangleProperties,
    pub passive_r_text: TextProperties,
    pub period: TextProperties,
    pub passive_counter_dec: TextProperties,
    pub passive_counter_sec: TextProperties,
    pub timer_m: TextProperties,
    pub timer_colon: TextProperties,
    pub timer_d: TextProperties,
    pub timer_s: TextProperties,
    pub passive_indicator: RectangleProperties,
    pub left_color_indicator: RectangleProperties,
    pub right_color_indicator: RectangleProperties,
    pub left_white_indicator: RectangleProperties,
    pub right_white_indicator: RectangleProperties,
    // pub recording_indicator: RectangleProperties,
    pub timer_text: TextProperties,

    pub cyrano_layout: Option<CyranoLayout>,
}

impl CyranoLayout {
    pub const fn empty_layout() -> Self {
        Self {
            left_name: TextProperties::get_empty(),
            left_nation: TextProperties::get_empty(),
            left_flag: RectangleProperties::get_empty(),
            left_score: TextProperties::get_empty(),
            left_status: TextProperties::get_empty(),
            left_video: TextProperties::get_empty(),
            left_medical: TextProperties::get_empty(),
            left_reserve: TextProperties::get_empty(),
            left_id: TextProperties::get_empty(),
            left_photo: RectangleProperties::get_empty(),

            right_name: TextProperties::get_empty(),
            right_nation: TextProperties::get_empty(),
            right_flag: RectangleProperties::get_empty(),
            right_score: TextProperties::get_empty(),
            right_status: TextProperties::get_empty(),
            right_video: TextProperties::get_empty(),
            right_medical: TextProperties::get_empty(),
            right_reserve: TextProperties::get_empty(),
            right_id: TextProperties::get_empty(),
            right_photo: RectangleProperties::get_empty(),

            referee_name: TextProperties::get_empty(),
            referee_nation: TextProperties::get_empty(),

            piste: TextProperties::get_empty(),
            competition_phase: TextProperties::get_empty(),
            competition_type: TextProperties::get_empty(),
            poule_tableau_id: TextProperties::get_empty(),

            start_time: TextProperties::get_empty(),
            local_time: TextProperties::get_empty(),

            state: TextProperties::get_empty(),
            status: TextProperties::get_empty(),

            static_piste: TextProperties::get_empty(),
            static_competition_type: TextProperties::get_empty(),
            static_competition_phase: TextProperties::get_empty(),
            static_poule_tableau_id: TextProperties::get_empty(),
            static_start_time: TextProperties::get_empty(),
            static_referee: TextProperties::get_empty(),
        }
    }
}
