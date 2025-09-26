use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NAME_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    referee_nation_widget: Label<'a>,

    referee_nation: String,
    referee_nation_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.referee_nation.font_size);

            Some(Self {
                referee_nation_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.referee_nation,
                    context.logger,
                ),

                referee_nation: "".to_string(),
                referee_nation_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.referee_nation != data.referee.nation {
            self.referee_nation = data.referee.nation.clone();
            self.referee_nation_updated = true;
        }
    }

    fn render(&mut self) {
        if self.referee_nation_updated {
            self.referee_nation_widget.render(
                format!("{}", self.referee_nation.clone()),
                FENCER_NAME_TEXT,
                None,
            );
            self.referee_nation_updated = false;
        }

        self.referee_nation_widget.draw();
    }
}
