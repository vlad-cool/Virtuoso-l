use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NAME_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    phase_widget: Label<'a>,

    phase: String,
    phase_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.competition_phase.font_size);

            Some(Self {
                phase_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.competition_phase,
                    context.logger,
                ),

                phase: "-".to_string(),
                phase_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        let phase: String = if data.poul_tab.chars().all(|c| c.is_ascii_digit()) {
            format!("{}", data.phase)
        } else {
            "tableau".to_string()
        };

        if self.phase != phase {
            self.phase = phase.clone();
            self.phase_updated = true;
        }
    }

    fn render(&mut self) {
        if self.phase_updated {
            self.phase_widget
                .render(format!("{}", self.phase), FENCER_NAME_TEXT, None);
            self.phase_updated = false;
        }

        self.phase_widget.draw();
    }
}
