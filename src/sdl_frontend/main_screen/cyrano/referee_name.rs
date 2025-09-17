use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NAME_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    referee_name_widget: Label<'a>,

    referee_name: String,
    referee_name_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.referee_name.font_size);

            Some(Self {
                referee_name_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.referee_name,
                    context.logger,
                ),

                referee_name: "".to_string(),
                referee_name_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.referee_name != data.referee.name {
            self.referee_name = data.referee.name.clone();
            self.referee_name_updated = true;
        }
    }

    fn render(&mut self) {
        if self.referee_name_updated {
            self.referee_name_widget
                .render(format!("Arbitre {}", self.referee_name.clone()), FENCER_NAME_TEXT, None);
            self.referee_name_updated = false;
        }

        self.referee_name_widget.draw();
    }
}
