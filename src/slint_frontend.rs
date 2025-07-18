use slint::{Timer, TimerMode, ToSharedString};

use crate::layouts::*;

const MESSAGE_DISPLAY_TIME: Duration = Duration::from_secs(2);

impl match_info::PassiveCard {
    fn get_slint_type(&self) -> i32 {
        match self {
            match_info::PassiveCard::None => 0,
            match_info::PassiveCard::Yellow(_) => 1,
            match_info::PassiveCard::Red(_) => 2,
            match_info::PassiveCard::Black(_) => 3,
        }
    }

    fn get_card_numbers(&self) -> i32 {
        match self {
            match_info::PassiveCard::None => 1,
            match_info::PassiveCard::Yellow(n) => *n as i32,
            match_info::PassiveCard::Red(n) => *n as i32,
            match_info::PassiveCard::Black(n) => *n as i32,
        }
    }
}

pub struct SlintFrontend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    hw_config: HardwareConfig,
    logger: Logger,
}

impl SlintFrontend {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        hw_config: HardwareConfig,
        logger: Logger,
    ) -> Self {
        Self {
            match_info,
            hw_config,
            logger,
        }
    }
}

impl modules::VirtuosoModule for SlintFrontend {
    fn run(&mut self) {
        let app: Result<Virtuoso, slint::PlatformError> = Virtuoso::new();

        let app: Virtuoso = match app {
            Ok(app) => app,
            Err(err) => {
                self.logger
                    .critical_error(format!("Failed to create slint frontend, error: {err}"));
                return;
            }
        };

        match self.hw_config.display.resolution {
            Resolution::Res1920X1080 => app.set_layout(LAYOUT_1920X1080),
            Resolution::Res1920X550 => app.set_layout(LAYOUT_1920X550),
            Resolution::Res1920X480 => app.set_layout(LAYOUT_1920X480),
            Resolution::Res1920X360 => app.set_layout(LAYOUT_1920X360),
        }

        let weak_app_1: slint::Weak<Virtuoso> = app.as_weak();

        let match_info_clone: Arc<Mutex<match_info::MatchInfo>> = self.match_info.clone();
        let mut match_info_modified_count: u32 = 0;

        let timer: Timer = Timer::default();
        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(50),
            move || {
                if let Some(app) = weak_app_1.upgrade() {
                    match_info_modified_count =
                        update_data(&match_info_clone, &app, match_info_modified_count);
                }
            },
        );

        if let Err(err) = app.run() {
            self.logger
                .critical_error(format!("Failed to run slint frontend, error: {err}"));
            return;
        }

        let mut match_info_data: MutexGuard<'_, match_info::MatchInfo> =
            self.match_info.lock().unwrap();
        match_info_data.program_state = match_info::ProgramState::Exiting;
    }
}

fn update_data(
    match_info: &Arc<Mutex<match_info::MatchInfo>>,
    app: &Virtuoso,
    match_info_modified_count: u32,
) -> u32 {
    let match_info_data: MutexGuard<'_, match_info::MatchInfo> = match_info.lock().unwrap();
    let time: i32 = match_info_data.timer_controller.get_millis() as i32;
    let passive_indicator: i32 = match_info_data.passive_timer.get_indicator() as i32;

    if match_info_data.display_message_updated.elapsed() > MESSAGE_DISPLAY_TIME {
        app.set_timer_text("".to_shared_string())
    }

    let modified_count: u32 = if match_info_data.modified_count == match_info_modified_count {
        match_info_modified_count
    } else {
        app.set_left_score(match_info_data.left_fencer.score as i32);
        app.set_right_score(match_info_data.right_fencer.score as i32);

        app.set_last_ten_seconds(match_info_data.last_ten_seconds);
        app.set_timer_running(match_info_data.timer_running);
        app.set_period(match_info_data.period as i32);

        app.set_weapon(match match_info_data.weapon {
            match_info::Weapon::Epee => 1,
            match_info::Weapon::Sabre => 2,
            match_info::Weapon::Fleuret => 3,
        });

        app.set_priority(match match_info_data.priority {
            match_info::Priority::Left => -1,
            match_info::Priority::None => 0,
            match_info::Priority::Right => 1,
        });

        app.set_left_color_led_on(match_info_data.left_fencer.color_light);
        app.set_left_white_led_on(match_info_data.left_fencer.white_light);
        app.set_right_color_led_on(match_info_data.right_fencer.color_light);
        app.set_right_white_led_on(match_info_data.right_fencer.white_light);

        // TODO make actual caution and penalty cards
        // app.set_left_caution(match_info_data.left_fencer.yellow_card > 0);
        // app.set_left_penalty(match_info_data.left_fencer.red_card > 0);
        // app.set_right_caution(match_info_data.right_fencer.yellow_card > 0);
        // app.set_right_penalty(match_info_data.right_fencer.red_card > 0);

        // TODO make actual passive cards
        // app.set_left_bot_pcard(match_info_data.left_fencer.p_card > 0);
        // app.set_left_top_pcard(match_info_data.left_fencer.p_card > 1);
        // app.set_right_bot_pcard(match_info_data.right_fencer.p_card > 0);
        // app.set_right_top_pcard(match_info_data.right_fencer.p_card > 1);

        app.set_left_pcard_type(match_info_data.left_fencer.passive_card.get_slint_type());
        app.set_left_pcard_number(match_info_data.left_fencer.passive_card.get_card_numbers());
        app.set_right_pcard_type(match_info_data.right_fencer.passive_card.get_slint_type());
        app.set_right_pcard_number(match_info_data.right_fencer.passive_card.get_card_numbers());

        app.set_auto_score_on(match_info_data.auto_score_on);
        app.set_auto_timer_on(match_info_data.auto_timer_on);

        app.set_left_fencer_name(match_info_data.left_fencer.name.to_shared_string());
        app.set_left_fencer_nation(match_info_data.left_fencer.nation.to_shared_string());
        app.set_right_fencer_name(match_info_data.right_fencer.name.to_shared_string());
        app.set_right_fencer_nation(match_info_data.right_fencer.nation.to_shared_string());

        app.set_passive_counter(if match_info_data.passive_timer.get_counter() <= 60 {
            match_info_data.passive_timer.get_counter() as i32
        } else {
            -1
        });

        app.set_is_online(match_info_data.cyrano_online);

        if match_info_data.display_message_updated.elapsed() < MESSAGE_DISPLAY_TIME {
            app.set_timer_text(match_info_data.display_message.to_shared_string())
        }

        match_info_data.modified_count
    };
    std::mem::drop(match_info_data);

    if time % 1000 > 500 {
        app.set_timer_flashing(true);
    } else {
        app.set_timer_flashing(false);
    }

    if time >= 10000 {
        let time: i32 = (time + 999) / 1000;
        let timer_m: i32 = time / 60;
        let time: i32 = time % 60;
        let timer_d: i32 = time / 10;
        let timer_s: i32 = time % 10;
        app.set_timer_m(timer_m);
        app.set_timer_d(timer_d);
        app.set_timer_s(timer_s);
    } else {
        app.set_timer_m(time / 1000);
        let time: i32 = time % 1000;
        app.set_timer_d(time / 100);
        app.set_timer_s((time % 100) / 10);
    }
    app.set_passive_indicator(passive_indicator);
    modified_count
}
