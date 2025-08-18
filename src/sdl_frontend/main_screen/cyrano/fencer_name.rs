use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NAME_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_name_widget: Label<'a>,
    right_name_widget: Label<'a>,

    left_name: String,
    left_name_updated: bool,
    right_name: String,
    right_name_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.left_name.font_size);

        Self {
            left_name_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.left_name,
                context.logger,
            ),
            right_name_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.right_name,
                context.logger,
            ),

            left_name: "".to_string(),
            left_name_updated: true,
            right_name: "".to_string(),
            right_name_updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.left_name != data.left_fencer.name {
            self.left_name = data.left_fencer.name.clone();
            self.left_name_updated = true;
        }
        if self.right_name != data.right_fencer.name {
            self.right_name = data.right_fencer.name.clone();
            self.right_name_updated = true;
        }
    }

    fn render(&mut self) {
        if self.left_name_updated {
            self.left_name_widget
                .render(self.left_name.clone(), FENCER_NAME_TEXT, None);
            self.left_name_updated = false;
        }
        if self.right_name_updated {
            self.right_name_widget
                .render(self.right_name.clone(), FENCER_NAME_TEXT, None);
            self.right_name_updated = false;
        }

        self.left_name_widget.draw();
        self.right_name_widget.draw();
    }
}
