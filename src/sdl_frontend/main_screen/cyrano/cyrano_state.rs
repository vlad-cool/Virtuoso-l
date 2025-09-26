use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::cyrano_server::State;
use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::FENCER_NAME_TEXT;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    state_widget: Label<'a>,

    state: State,
    state_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.state.font_size);

            Some(Self {
                state_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.state,
                    context.logger,
                ),

                state: State::Waiting,
                state_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.state != data.cyrano_state {
            self.state = data.cyrano_state;
            self.state_updated = true;
        }
    }

    fn render(&mut self) {
        if self.state_updated {
            self.state_widget
                .render(format!("State {}", self.state), FENCER_NAME_TEXT, None);
            self.state_updated = false;
        }

        self.state_widget.draw();
    }
}
