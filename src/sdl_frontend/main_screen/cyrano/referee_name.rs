use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::{FENCER_NAME_TEXT, STATIC_TEXT, STATIC_TEXT_GRAY};
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    referee_name_widget: Label<'a>,
    referee_static_widget: Label<'a>,

    referee_name: String,
    referee_name_updated: bool,
    referee_active: bool,
    referee_active_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.referee_name.font_size);
            let font_static: Rc<Font<'_, '_>> = context.get_font(layout.static_referee.font_size);

            Some(Self {
                referee_name_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.referee_name,
                    context.logger,
                ),
                referee_static_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font_static.clone(),
                    layout.static_referee,
                    context.logger,
                ),

                referee_name: "".to_string(),
                referee_name_updated: true,
                referee_active: false,
                referee_active_updated: true,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.referee_name != data.referee.name {
            self.referee_name = data.referee.name.clone();
            self.referee_name_updated = true;
        }

        if data.referee.name.trim() != "" || data.referee.nation.trim() != "" {
            if !self.referee_active {
                self.referee_active_updated = true;
            }
            self.referee_active = true;
        } else {
            if self.referee_active {
                self.referee_active_updated = true;
            }
            self.referee_active = false;
        }
    }

    fn render(&mut self) {
        if self.referee_name_updated {
            self.referee_name_widget.render(
                format!("{}", self.referee_name.clone()),
                FENCER_NAME_TEXT,
                None,
            );
            self.referee_name_updated = false;
        }

        if self.referee_active_updated {
            self.referee_static_widget.render(
                "Arbitre".to_string(),
                if self.referee_active {
                    STATIC_TEXT
                } else {
                    STATIC_TEXT_GRAY
                },
                None,
            );
            self.referee_active_updated = false;
        }

        self.referee_name_widget.draw();
        self.referee_static_widget.draw();
    }
}
