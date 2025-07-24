use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::time::Duration;

use crate::sdl_frontend::colors;
use crate::match_info::{MatchInfo, Priority};
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    timer_0_widget: Label<'a>,
    timer_1_widget: Label<'a>,
    timer_2_widget: Label<'a>,
    timer_3_widget: Label<'a>,

    time: Duration,
    timer_running: bool,
    priority: Priority,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.timer_m.font_size);

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
            time: Duration::from_secs(3 * 60),
            timer_running: false,
            priority: Priority::None,
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        let time: std::time::Duration = data.timer_controller.get_time();
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
        if self.updated {
            let colon: String = if !self.timer_running || self.time.subsec_millis() > 500 {
                ":".to_string()
            } else {
                " ".to_string()
            };

            let time_str: String = if self.time.as_secs() >= 10 {
                let minutes: u64 = self.time.as_secs() / 60;
                let seconds: u64 = self.time.as_secs() % 60;

                format!("{}{}{}{}", minutes, colon, seconds / 10, seconds % 10)
            } else {
                let seconds: u64 = self.time.as_secs();
                let centiseconds: u32 = self.time.subsec_millis() / 10;

                format!(
                    "{}{}{}{}",
                    seconds,
                    colon,
                    centiseconds / 10,
                    centiseconds % 10
                )
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

            self.timer_0_widget.render(&time_str[0..1], color);
            self.timer_1_widget.render(&time_str[1..2], colon_color);
            self.timer_2_widget.render(&time_str[2..3], color);
            self.timer_3_widget.render(&time_str[3..4], color);
            self.updated = false;
        }
        self.timer_0_widget.draw();
        self.timer_1_widget.draw();
        self.timer_2_widget.draw();
        self.timer_3_widget.draw();
    }
}
