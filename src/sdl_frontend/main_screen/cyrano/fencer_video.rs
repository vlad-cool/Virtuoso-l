use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::{COLOR_LABELS_RED, COLOR_LABELS_GREEN};
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_status_widget: Label<'a>,
    right_status_widget: Label<'a>,

    left_status: u32,
    left_status_updated: bool,
    right_status: u32,
    right_status_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.left_video.font_size);

            Some(Self {
                left_status_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.left_video,
                    context.logger,
                ),
                right_status_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.right_video,
                    context.logger,
                ),

                left_status: 0,
                left_status_updated: true,
                right_status: 0,
                right_status_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.left_status != data.left_fencer.video_appeal {
            self.left_status = data.left_fencer.video_appeal;
            self.left_status_updated = true;
        }
        if self.right_status != data.right_fencer.video_appeal {
            self.right_status = data.right_fencer.video_appeal;
            self.right_status_updated = true;
        }
    }

    fn render(&mut self) {
        if self.left_status_updated {
            self.left_status_widget.render(
                format!("{}", self.left_status),
                COLOR_LABELS_RED,
                None,
            );
            self.left_status_updated = false;
        }
        if self.right_status_updated {
            self.right_status_widget.render(
                format!("{}", self.right_status),
                COLOR_LABELS_GREEN,
                None,
            );
            self.right_status_updated = false;
        }

        self.left_status_widget.draw();
        self.right_status_widget.draw();
    }
}
