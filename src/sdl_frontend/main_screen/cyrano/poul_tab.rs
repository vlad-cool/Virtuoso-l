use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NAME_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    poul_tab_widget: Label<'a>,

    poul_tab: String,
    poul_tab_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.poule_tableau_id.font_size);

            Some(Self {
                poul_tab_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.poule_tableau_id,
                    context.logger,
                ),

                poul_tab: "".to_string(),
                poul_tab_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        let poul_tab: String = if data.poul_tab.chars().all(|c| c.is_ascii_digit()) {
            format!("{}", data.poul_tab)
        } else {
            format!("{}", data.poul_tab)
        };

        if self.poul_tab != poul_tab {
            self.poul_tab = data.poul_tab.clone();
            self.poul_tab_updated = true;
        }
    }

    fn render(&mut self) {
        if self.poul_tab_updated {
            self.poul_tab_widget
                .render(self.poul_tab.clone(), FENCER_NAME_TEXT, None);
            self.poul_tab_updated = false;
        }

        self.poul_tab_widget.draw();
    }
}
