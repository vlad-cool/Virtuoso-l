use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::{CompetitionType, MatchInfo};
use crate::gui_frontend::colors::FENCER_NAME_TEXT;
use crate::gui_frontend::widgets::Label;
use crate::gui_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    competition_type_widget: Label<'a>,

    competition_type: CompetitionType,
    competition_type_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.competition_type.font_size);

            Some(Self {
                competition_type_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.competition_type,
                    context.logger,
                ),

                competition_type: CompetitionType::Individual,
                competition_type_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if let Some(competition_type) = data.competition_type
            && self.competition_type != competition_type
        {
            self.competition_type = competition_type;
            self.competition_type_updated = true;
        }
    }

    fn render(&mut self) {
        if self.competition_type_updated {
            self.competition_type_widget.render(
                format!("Competition type {}", self.competition_type),
                FENCER_NAME_TEXT,
                None,
            );
            self.competition_type_updated = false;
        }

        self.competition_type_widget.draw();
    }
}
