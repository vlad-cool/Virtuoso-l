use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::WHITE_LABELS_LIGHT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_reserve_widget: Label<'a>,
    right_reserve_widget: Label<'a>,

    left_reserve: bool,
    left_reserve_updated: bool,
    right_reserve: bool,
    right_reserve_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.left_reserve.font_size);

            Some(Self {
                left_reserve_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.left_reserve,
                    context.logger,
                ),
                right_reserve_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.right_reserve,
                    context.logger,
                ),

                left_reserve: false,
                left_reserve_updated: true,
                right_reserve: false,
                right_reserve_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.left_reserve != data.left_fencer.reserve_introduction {
            self.left_reserve = data.left_fencer.reserve_introduction;
            self.left_reserve_updated = true;
        }
        if self.right_reserve != data.right_fencer.reserve_introduction {
            self.right_reserve = data.right_fencer.reserve_introduction;
            self.right_reserve_updated = true;
        }
    }

    fn render(&mut self) {
        if self.left_reserve_updated {
            self.left_reserve_widget.render(
                format!("{}", if self.left_reserve { "R" } else { "N" }),
                WHITE_LABELS_LIGHT,
                None,
            );
            self.left_reserve_updated = false;
        }
        if self.right_reserve_updated {
            self.right_reserve_widget.render(
                format!("{}", if self.right_reserve { "R" } else { "N" }),
                WHITE_LABELS_LIGHT,
                None,
            );
            self.right_reserve_updated = false;
        }

        self.left_reserve_widget.draw();
        self.right_reserve_widget.draw();
    }
}
