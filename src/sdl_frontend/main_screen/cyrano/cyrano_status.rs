use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::*;
use crate::sdl_frontend::layout_structure::TextProperties;
use crate::sdl_frontend::widgets::Card;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    status_widget: Card<'a>,

    online: bool,
    active: bool,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(layout.left_nation.font_size);

        Self {
            status_widget: Card::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                TextProperties {
                    x: context.layout.recording_indicator.x,
                    y: context.layout.recording_indicator.y,
                    width: context.layout.recording_indicator.width,
                    height: context.layout.recording_indicator.height,
                    font_size: 1,
                },
                context.layout.recording_indicator,
                context.logger,
            ),

            online: false,
            active: false,
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.online != data.cyrano_online || self.active != data.cyrano_active {
            self.online = data.cyrano_online;
            self.active = data.cyrano_active;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated {
            let online_color: sdl2::pixels::Color = if self.online {
                CYRANO_ONLINE
            } else {
                BACKGROUND
            };
            let active_color: sdl2::pixels::Color = if self.active {
                CYRANO_ACTIVE
            } else {
                online_color
            };

            self.status_widget
                .render(" ", 2, active_color, online_color, BACKGROUND);
            self.updated = false;
        }

        self.status_widget.draw();
    }
}
