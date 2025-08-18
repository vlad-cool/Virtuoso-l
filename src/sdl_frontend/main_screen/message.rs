use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::{Label, LabelTextureCache};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    message_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    message: String,
    update_time: Option<Instant>,
    message_updated: bool,
}

impl<'a> Drawer<'a> {
    pub const MESSAGE_DISPLAY_TIME: Duration = Duration::from_secs(2);

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
            texture_cache: LabelTextureCache::new(),
            message: "".to_string(),
            update_time: None,
            message_updated: false,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.message != data.display_message {
            self.update_time = data.display_message_updated;
            self.message = data.display_message.clone();
            self.message_updated = true;
        }
    }

    fn render(&mut self) {
        if self.message_updated {
            self.message_widget.render(
                self.message.clone(),
                colors::TIMER_ORANGE,
                Some(&mut self.texture_cache),
            );
            self.message_updated = false;
        }
        if let Some(update_time) = self.update_time
            && update_time.elapsed() < Self::MESSAGE_DISPLAY_TIME
        {
            self.message_widget.draw();
            eprintln!("N>NJHJGFD");
        }
    }
}
