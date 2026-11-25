use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::{INACTIVE, FENCER_NATION_TEXT};
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
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.left_nation.font_size);

            Some(Self {
                left_nation_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.left_nation,
                    context.logger,
                ),
                right_nation_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.right_nation,
                    context.logger,
                ),

                left_nation: "".to_string(),
                left_nation_updated: true,
                right_nation: "".to_string(),
                right_nation_updated: true,
            })
        } else {
            None
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
        let (left_fencer, left_color) = if self.left_nation == "" {
            (&"CIT".to_string(), INACTIVE)
        } else {
            (&self.left_nation, FENCER_NATION_TEXT)
        };
        let (right_fencer, right_color) = if self.right_nation == "" {
            (&"CIT".to_string(), INACTIVE)
        } else {
            (&self.right_nation, FENCER_NATION_TEXT)
        };

        if self.left_nation_updated {
            self.left_nation_widget
                .render(left_fencer.clone(), left_color, None);
            self.left_nation_updated = false;
        }
        if self.right_nation_updated {
            self.right_nation_widget
                .render(right_fencer.clone(), right_color, None);
            self.right_nation_updated = false;
        }

        self.left_nation_widget.draw();
        self.right_nation_widget.draw();
    }
}
