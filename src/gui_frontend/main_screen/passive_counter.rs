use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::{MatchInfo, TimerController};
use crate::gui_frontend::colors;
use crate::gui_frontend::widgets::{Label, LabelTextureCache};
use crate::gui_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    passive_counter_0_widget: Label<'a>,
    passive_counter_1_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    timer: TimerController,

    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.passive_counter_dec.font_size);

        Self {
            passive_counter_0_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.passive_counter_dec,
                context.logger,
            ),
            passive_counter_1_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.passive_counter_sec,
                context.logger,
            ),
            texture_cache: LabelTextureCache::new(),
            timer: TimerController::new(),
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.timer != data.timer_controller {
            self.timer = data.timer_controller;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated {
            let passive_counter_text: String = self.timer.get_passive_counter();

            let color: sdl2::pixels::Color = if self.timer.is_passive_timer_enabled() {
                colors::PASSIVE_TEXT_LIGHT
            } else {
                colors::PASSIVE_TEXT_DARK
            };

            self.passive_counter_0_widget.render(
                passive_counter_text[0..1].to_string(),
                color,
                Some(&mut self.texture_cache),
            );
            self.passive_counter_1_widget.render(
                passive_counter_text[1..2].to_string(),
                color,
                Some(&mut self.texture_cache),
            );
            self.updated = false;
        }

        self.passive_counter_0_widget.draw();
        self.passive_counter_1_widget.draw();
    }
}
