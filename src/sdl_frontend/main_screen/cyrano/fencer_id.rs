use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NAME_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_id_widget: Label<'a>,
    right_id_widget: Label<'a>,

    left_id: u32,
    left_id_updated: bool,
    right_id: u32,
    right_id_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.left_id.font_size);

            Some(Self {
                left_id_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.left_id,
                    context.logger,
                ),
                right_id_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.right_id,
                    context.logger,
                ),

                left_id: 0,
                left_id_updated: true,
                right_id: 0,
                right_id_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.left_id != data.left_fencer.id {
            self.left_id = data.left_fencer.id.clone();
            self.left_id_updated = true;
        }
        if self.right_id != data.right_fencer.id {
            self.right_id = data.right_fencer.id.clone();
            self.right_id_updated = true;
        }
    }

    fn render(&mut self) {
        if self.left_id_updated {
            self.left_id_widget
                .render(format!("{} FIE ID", self.left_id), FENCER_NAME_TEXT, None);
            self.left_id_updated = false;
        }
        if self.right_id_updated {
            self.right_id_widget.render(
                format!("{} FIE ID", self.right_id),
                FENCER_NAME_TEXT,
                None,
            );
            self.right_id_updated = false;
        }

        self.left_id_widget.draw();
        self.right_id_widget.draw();
    }
}
