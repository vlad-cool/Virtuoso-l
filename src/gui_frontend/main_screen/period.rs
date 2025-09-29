use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::gui_frontend::colors;
use crate::gui_frontend::widgets::{Label, LabelTextureCache};
use crate::gui_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    period_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    period: u32,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.passive_counter_dec.font_size);

        Self {
            period_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.period,
                context.logger,
            ),
            texture_cache: LabelTextureCache::new(),

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
                .render(period_text, colors::PERIOD, Some(&mut self.texture_cache));
            self.updated = false;
        }
        self.period_widget.draw();
    }
}
