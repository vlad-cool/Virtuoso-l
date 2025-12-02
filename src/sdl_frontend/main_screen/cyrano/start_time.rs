use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::{FENCER_NAME_TEXT, STATIC_TEXT, STATIC_TEXT_GRAY};
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    start_time_widget: Label<'a>,
    start_time_static_widget: Label<'a>,

    start_time: String,
    start_time_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.start_time.font_size);
            let font_static: Rc<Font<'_, '_>> = context.get_font(layout.static_start_time.font_size);

            Some(Self {
                start_time_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.start_time,
                    context.logger,
                ),
                start_time_static_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font_static.clone(),
                    layout.static_start_time,
                    context.logger,
                ),

                start_time: "".to_string(),
                start_time_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.start_time != data.time {
            self.start_time = data.time.clone();
            self.start_time_updated = true;
        }
    }

    fn render(&mut self) {
        if self.start_time_updated {
            self.start_time_widget
                .render(self.start_time.clone(), FENCER_NAME_TEXT, None);
            self.start_time_static_widget.render(
                "Time to start the match".to_string(),
                if self.start_time == "" {
                    STATIC_TEXT_GRAY
                } else {
                    STATIC_TEXT
                },
                None,
            );
            self.start_time_updated = false;
        }

        self.start_time_widget.draw();
        self.start_time_static_widget.draw();
    }
}
