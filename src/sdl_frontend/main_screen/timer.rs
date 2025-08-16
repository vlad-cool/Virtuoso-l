use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::time::Duration;

use crate::match_info::{MatchInfo, Priority};
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::{Label, LabelHashKey, LabelTextureCache};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

fn digit_to_str(digit: u64) -> String {
    match digit {
        0 => "0".to_string(),
        1 => "1".to_string(),
        2 => "2".to_string(),
        3 => "3".to_string(),
        4 => "4".to_string(),
        5 => "5".to_string(),
        6 => "6".to_string(),
        7 => "7".to_string(),
        8 => "8".to_string(),
        9 => "9".to_string(),
        10.. => "-".to_string(),
    }
}

pub struct Drawer<'a> {
    timer_0_widget: Label<'a>,
    timer_1_widget: Label<'a>,
    timer_2_widget: Label<'a>,
    timer_3_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    time: Duration,
    timer_running: bool,
    priority: Priority,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.timer_m.font_size);
        let mut texture_cache: LabelTextureCache<'a> = LabelTextureCache::new();

        for char in "0123456789".chars() {
            for color in [
                colors::TIMER_WHITE,
                colors::TIMER_ORANGE,
                colors::TIMER_BLUE,
            ] {
                let key: LabelHashKey = LabelHashKey {
                    color,
                    text: char.to_string(),
                };
                texture_cache.get(key, context.texture_creator, font.clone(), context.logger);
            }
        }
        for char in " :".chars() {
            for color in [
                colors::TIMER_WHITE,
                colors::TIMER_ORANGE,
                colors::TIMER_BLUE,
                colors::PRIORITY_RED,
                colors::PRIORITY_GREEN,
            ] {
                let key: LabelHashKey = LabelHashKey {
                    color,
                    text: char.to_string(),
                };
                texture_cache.get(key, context.texture_creator, font.clone(), context.logger);
            }
        }

        Self {
            timer_0_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.timer_m,
                context.logger,
            ),
            timer_1_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.timer_colon,
                context.logger,
            ),
            timer_2_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.timer_d,
                context.logger,
            ),
            timer_3_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.timer_s,
                context.logger,
            ),
            texture_cache,

            time: Duration::from_secs(3 * 60),
            timer_running: false,
            priority: Priority::None,
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        let time: std::time::Duration =
            data.timer_controller.get_time() + Duration::from_millis(999);
        if self.time != time
            || self.timer_running != data.timer_running
            || self.priority != data.priority
        {
            self.time = time;
            self.timer_running = data.timer_running;
            self.priority = data.priority;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated || self.timer_running {
            let colon: String = if !self.timer_running || self.time.subsec_millis() > 500 {
                ":".to_string()
            } else {
                " ".to_string()
            };

            let (timer_m, timer_s) = if (self.time + Duration::from_millis(999)).as_secs() >= 10 {
                let minutes: u64 = self.time.as_secs() / 60;
                let seconds: u64 = self.time.as_secs() % 60;

                (minutes, seconds)
            } else {
                let seconds: u64 = self.time.as_secs();
                let centiseconds: u32 = self.time.subsec_millis() / 10;

                (seconds, centiseconds as u64)
            };

            let color: sdl2::pixels::Color = if self.timer_running {
                if self.time.as_secs() > 10 {
                    colors::TIMER_WHITE
                } else {
                    colors::TIMER_BLUE
                }
            } else {
                colors::TIMER_ORANGE
            };

            let colon_color: sdl2::pixels::Color = if self.timer_running {
                match self.priority {
                    Priority::Left => colors::PRIORITY_RED,
                    Priority::None => color,
                    Priority::Right => colors::PRIORITY_GREEN,
                }
            } else {
                color
            };

            self.timer_0_widget.render(
                digit_to_str(timer_m % 10),
                color,
                Some(&mut self.texture_cache),
            );
            self.timer_1_widget
                .render(colon, colon_color, Some(&mut self.texture_cache));
            self.timer_2_widget.render(
                digit_to_str(timer_s / 10),
                color,
                Some(&mut self.texture_cache),
            );
            self.timer_3_widget.render(
                digit_to_str(timer_s % 10),
                color,
                Some(&mut self.texture_cache),
            );
            self.updated = false;
        }
        self.timer_0_widget.draw();
        self.timer_1_widget.draw();
        self.timer_2_widget.draw();
        self.timer_3_widget.draw();
    }
}
