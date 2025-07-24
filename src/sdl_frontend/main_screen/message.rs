use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::time::Duration;

use crate::sdl_frontend::colors;
use crate::match_info::MatchInfo;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    message_widget: Label<'a>,

    message: String,
    display: bool,
    message_updated: bool,
}

impl<'a> Drawer<'a> {
    const MESSAGE_DISPLAY_TIME: Duration = Duration::from_secs(2);

    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.timer_text.font_size);

        Self {
            message_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.timer_text,
                context.logger,
            ),
            message: "".to_string(),
            display: false,
            message_updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        self.display = data.display_message_updated.elapsed() < Self::MESSAGE_DISPLAY_TIME;

        if self.message != data.display_message {
            self.message = data.display_message.clone();
            self.message_updated = true;
        }
    }

    fn render(&mut self) {
        if self.message_updated {
            self.message_widget
                .render(self.message.as_str(), colors::TIMER_ORANGE);
            self.message_updated = false;
        }
        if self.display {
            self.message_widget.draw();
        }
    }
}
