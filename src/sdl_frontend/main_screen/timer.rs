use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::match_info::{MatchInfo, Priority, TimerController};
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::{Label, LabelHashKey, LabelTextureCache};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    timer_0_widget: Label<'a>,
    timer_1_widget: Label<'a>,
    timer_2_widget: Label<'a>,
    timer_3_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    timer: TimerController,
    priority: Priority,
    updated: bool,
    message_update_time: Option<Instant>,
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
        for char in " :.".chars() {
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

            timer: TimerController::new(),
            priority: Priority::None,
            updated: true,
            message_update_time: None,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        self.message_update_time = data.display_message_updated;

        let timer: TimerController = data.timer_controller;
        if self.timer != timer || self.priority != data.priority {
            self.timer = timer;
            self.priority = data.priority;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated || self.timer.is_timer_running() {
            let (time_string, time): (String, Duration) = self.timer.get_main_time_string();

            let color: sdl2::pixels::Color = if time.as_secs() >= 10 {
                if self.timer.is_timer_running() {
                    colors::TIMER_WHITE
                } else {
                    colors::TIMER_ORANGE
                }
            } else {
                colors::TIMER_BLUE
            };

            let colon_color: sdl2::pixels::Color = if self.timer.is_timer_running() {
                if time.subsec_millis() > 500 {
                    match self.priority {
                        Priority::Left => colors::PRIORITY_RED,
                        Priority::None => color,
                        Priority::Right => colors::PRIORITY_GREEN,
                    }
                } else {
                    colors::BACKGROUND
                }
            } else {
                color
            };

            self.timer_0_widget.render(
                time_string[0..1].to_string(),
                color,
                Some(&mut self.texture_cache),
            );
            self.timer_1_widget.render(
                time_string[1..2].to_string(),
                colon_color,
                Some(&mut self.texture_cache),
            );
            self.timer_2_widget.render(
                time_string[2..3].to_string(),
                color,
                Some(&mut self.texture_cache),
            );
            self.timer_3_widget.render(
                time_string[3..4].to_string(),
                color,
                Some(&mut self.texture_cache),
            );
            self.updated = false;
        }

        if let Some(update_time) = self.message_update_time
            && update_time.elapsed() == Duration::ZERO
        {
        } else {
            self.timer_0_widget.draw();
            self.timer_1_widget.draw();
            self.timer_2_widget.draw();
            self.timer_3_widget.draw();
        }
    }
}
