use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::colors;
use crate::match_info::MatchInfo;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    period_widget: Label<'a>,

    period: u32,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> =
            context.get_font(context.layout.passive_counter_dec.font_size as u16);

        Self {
            period_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.period,
                context.logger,
            ),
            period: 1,
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.period != data.period {
            self.period = data.period;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated {
            let period_text: String = format!("{}", self.period % 10);
            self.period_widget
                .render(&period_text.as_str(), colors::PERIOD);
            self.updated = false;
        }
        self.period_widget.draw();
    }
}
