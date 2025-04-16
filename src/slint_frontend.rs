use slint::{Timer, TimerMode};
use std::sync::{Arc, Mutex};

use crate::match_info;
use crate::modules;

use crate::layouts::*;

pub struct SlintFrontend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
}

impl SlintFrontend {
    pub fn new(match_info: Arc<Mutex<match_info::MatchInfo>>) -> Self {
        Self {
            match_info: match_info,
        }
    }
}

impl modules::VirtuosoModule for SlintFrontend {
    fn run(&mut self) {
        let app = Virtuoso::new().unwrap();

        app.set_layout(LAYOUT_1920X480);

        let weak_app_1 = app.as_weak();
        let weak_app_2 = app.as_weak();

        let match_info_clone = self.match_info.clone();
        let mut match_info_modified_count = 0u32;

        let timer = Timer::default();
        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(100),
            move || {
                if let Some(app) = weak_app_1.upgrade() {
                    let seconds_updated: bool;
                    (match_info_modified_count, seconds_updated) =
                        update_data(&match_info_clone, &app, match_info_modified_count);

                    if seconds_updated {
                        app.set_timer_flashing(true);
                        let weak_app_3 = weak_app_2.clone();
                        let flash_timer = Timer::default();
                        flash_timer.start(
                            TimerMode::SingleShot,
                            std::time::Duration::from_millis(500),
                            move || {
                                if let Some(app) = weak_app_3.upgrade() {
                                    app.set_timer_flashing(false);
                                }
                            },
                        );
                    }
                }
            },
        );

        app.run().unwrap();

        let mut match_info_data = self.match_info.lock().unwrap();
        match_info_data.program_state = match_info::ProgramState::Exiting;
    }

    fn get_module_type(&self) -> modules::Modules {
        modules::Modules::SlintFrontend
    }
}

fn update_data(
    match_info: &Arc<Mutex<match_info::MatchInfo>>,
    app: &Virtuoso,
    match_info_modified_count: u32,
) -> (u32, bool) {
    let match_info_data = match_info.lock().unwrap();

    if match_info_data.modified_count == match_info_modified_count {
        return (match_info_modified_count, false);
    } else {
        let seconds_updated: bool;
        if (app.get_timer() % 10) as u32 != match_info_data.timer % 10 {
            // TODO Add last ten seconds support
            seconds_updated = true;
            println!("Seconds updated");
        } else {
            seconds_updated = false;
        }

        app.set_left_score(match_info_data.left_score as i32);
        app.set_right_score(match_info_data.right_score as i32);
        app.set_timer(match_info_data.timer as i32);
        app.set_last_ten_seconds(match_info_data.last_ten_seconds);
        app.set_timer_running(match_info_data.timer_running);
        app.set_period(match_info_data.period as i32);

        app.set_weapon(match match_info_data.weapon {
            match_info::Weapon::Unknown => 0,
            match_info::Weapon::Epee => 1,
            match_info::Weapon::Sabre => 2,
            match_info::Weapon::Fleuret => 3,
        });

        app.set_priority(match match_info_data.priority {
            match_info::Priority::Left => -1,
            match_info::Priority::None => 0,
            match_info::Priority::Right => 1,
        });

        app.set_left_color_led_on(match_info_data.left_red_led_on);
        app.set_left_white_led_on(match_info_data.left_white_led_on);
        app.set_right_color_led_on(match_info_data.right_green_led_on);
        app.set_right_white_led_on(match_info_data.right_white_led_on);

        app.set_left_caution(match_info_data.left_caution);
        app.set_left_penalty(match_info_data.left_penalty);
        app.set_right_caution(match_info_data.right_caution);
        app.set_right_penalty(match_info_data.right_penalty);

        app.set_left_bot_pcard(match_info_data.left_pcard_bot);
        app.set_left_top_pcard(match_info_data.left_pcard_top);
        app.set_right_bot_pcard(match_info_data.right_pcard_bot);
        app.set_right_top_pcard(match_info_data.right_pcard_top);

        app.set_auto_score_on(match_info_data.auto_score_on);
        app.set_auto_timer_on(match_info_data.auto_timer_on);

        app.set_passive_counter(if match_info_data.passive_counter <= 60 {
            match_info_data.passive_counter as i32
        } else {
            -1
        });
        app.set_passive_indicator(match_info_data.passive_indicator as i32);

        app.set_is_online(match_info_data.cyrano_online);
        return (match_info_data.modified_count, seconds_updated);
    }
}
