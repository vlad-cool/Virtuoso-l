use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::{Label, LabelTextureCache};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    timer_widget: Label<'a>,
    score_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    status_only: bool,

    auto_timer_on: bool,
    auto_timer_updated: bool,
    auto_score_on: bool,
    auto_score_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.auto_timer_status.font_size);

        Self {
            score_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.auto_score_status,
                context.logger,
            ),
            timer_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.auto_timer_status,
                context.logger,
            ),
            texture_cache: LabelTextureCache::new(),

            status_only: context.layout.auto_status_only,

            auto_score_on: false,
            auto_score_updated: true,
            auto_timer_on: false,
            auto_timer_updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.auto_timer_on != data.auto_timer_on {
            self.auto_timer_on = data.auto_timer_on;
            self.auto_timer_updated = true;
        }
        if self.auto_score_on != data.auto_score_on {
            self.auto_score_on = data.auto_score_on;
            self.auto_score_updated = true;
        }
    }

    fn render(&mut self) {
        if self.auto_timer_updated {
            let prefix: &'static str = if self.status_only { "" } else { "auto\ntimer " };

            let (text, color) = if self.auto_timer_on {
                (format!("{}on", prefix), colors::AUTO_STATUS_TEXT_LIGHT)
            } else {
                (format!("{}off", prefix), colors::AUTO_STATUS_TEXT_DARK)
            };

            self.timer_widget
                .render(text, color, Some(&mut self.texture_cache));
            self.auto_timer_updated = false;
        }

        if self.auto_score_updated {
            let prefix: &'static str = if self.status_only { "" } else { "auto\nscore " };

            let (text, color) = if self.auto_score_on {
                (format!("{}on", prefix), colors::AUTO_STATUS_TEXT_LIGHT)
            } else {
                (format!("{}off", prefix), colors::AUTO_STATUS_TEXT_DARK)
            };

            self.score_widget
                .render(text, color, Some(&mut self.texture_cache));
            self.auto_score_updated = false;
        }

        self.timer_widget.draw();
        self.score_widget.draw();
    }
}
