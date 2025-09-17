use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NAME_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    piste_widget: Label<'a>,

    piste: String,
    piste_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.piste.font_size);

            Some(Self {
                piste_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.piste,
                    context.logger,
                ),

                piste: "".to_string(),
                piste_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.piste != data.piste {
            self.piste = data.piste.clone();
            self.piste_updated = true;
        }
    }

    fn render(&mut self) {
        if self.piste_updated {
            self.piste_widget
                .render(self.piste.clone(), FENCER_NAME_TEXT, None);
            self.piste_updated = false;
        }

        self.piste_widget.draw();
    }
}
