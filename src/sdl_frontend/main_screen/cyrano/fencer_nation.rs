use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NATION_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_nation_widget: Label<'a>,
    right_nation_widget: Label<'a>,

    left_nation: String,
    left_nation_updated: bool,
    right_nation: String,
    right_nation_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.left_nation.font_size);

        Self {
            left_nation_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.left_nation,
                context.logger,
            ),
            right_nation_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.right_nation,
                context.logger,
            ),

            left_nation: "".to_string(),
            left_nation_updated: true,
            right_nation: "".to_string(),
            right_nation_updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.left_nation != data.left_fencer.nation {
            self.left_nation = data.left_fencer.nation.clone();
            self.left_nation_updated = true;
        }
        if self.right_nation != data.right_fencer.nation {
            self.right_nation = data.right_fencer.nation.clone();
            self.right_nation_updated = true;
        }
    }

    fn render(&mut self) {
        if self.left_nation_updated {
            self.left_nation_widget
                .render(self.left_nation.clone(), FENCER_NATION_TEXT, None);
            self.left_nation_updated = false;
        }
        if self.right_nation_updated {
            self.right_nation_widget
                .render(self.right_nation.clone(), FENCER_NATION_TEXT, None);
            self.right_nation_updated = false;
        }

        self.left_nation_widget.draw();
        self.right_nation_widget.draw();
    }
}
