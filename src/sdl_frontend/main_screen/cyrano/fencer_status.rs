use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::{FencerStatus, MatchInfo};
use crate::sdl_frontend::colors::{FENCER_NATION_TEXT, WHITE_LABELS_LIGHT};
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_status_widget: Label<'a>,
    right_status_widget: Label<'a>,

    left_status: FencerStatus,
    left_status_updated: bool,
    right_status: FencerStatus,
    right_status_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.left_status.font_size);

            Some(Self {
                left_status_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.left_status,
                    context.logger,
                ),
                right_status_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.right_status,
                    context.logger,
                ),

                left_status: FencerStatus::Undefined,
                left_status_updated: true,
                right_status: FencerStatus::Undefined,
                right_status_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.left_status != data.left_fencer.status {
            self.left_status = data.left_fencer.status;
            self.left_status_updated = true;
        }
        if self.right_status != data.right_fencer.status {
            self.right_status = data.right_fencer.status;
            self.right_status_updated = true;
        }
    }

    fn render(&mut self) {
        if self.left_status_updated {
            self.left_status_widget.render(
                format!("{:1} Status", self.left_status),
                WHITE_LABELS_LIGHT,
                None,
            );
            self.left_status_updated = false;
        }
        if self.right_status_updated {
            self.right_status_widget.render(
                format!("{:1} Status", self.right_status),
                FENCER_NATION_TEXT,
                None,
            );
            self.right_status_updated = false;
        }

        self.left_status_widget.draw();
        self.right_status_widget.draw();
    }
}
