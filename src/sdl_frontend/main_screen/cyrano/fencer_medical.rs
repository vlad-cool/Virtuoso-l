use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::{FENCER_NATION_TEXT, WHITE_LABELS_LIGHT};
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_medical_widget: Label<'a>,
    right_medical_widget: Label<'a>,

    left_medical: u32,
    left_medical_updated: bool,
    right_medical: u32,
    right_medical_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.left_medical.font_size);

            Some(Self {
                left_medical_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.left_medical,
                    context.logger,
                ),
                right_medical_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.right_medical,
                    context.logger,
                ),

                left_medical: 0,
                left_medical_updated: true,
                right_medical: 0,
                right_medical_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.left_medical != data.left_fencer.medical_interventions {
            self.left_medical = data.left_fencer.medical_interventions;
            self.left_medical_updated = true;
        }
        if self.right_medical != data.right_fencer.medical_interventions {
            self.right_medical = data.right_fencer.medical_interventions;
            self.right_medical_updated = true;
        }
    }

    fn render(&mut self) {
        if self.left_medical_updated {
            self.left_medical_widget.render(
                format!("{}", self.left_medical),
                WHITE_LABELS_LIGHT,
                None,
            );
            self.left_medical_updated = false;
        }
        if self.right_medical_updated {
            self.right_medical_widget.render(
                format!("{}", self.right_medical),
                FENCER_NATION_TEXT,
                None,
            );
            self.right_medical_updated = false;
        }

        self.left_medical_widget.draw();
        self.right_medical_widget.draw();
    }
}
