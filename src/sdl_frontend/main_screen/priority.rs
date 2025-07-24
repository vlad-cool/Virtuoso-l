use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::{MatchInfo, Priority};
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    l_cap_widget: Label<'a>,
    l_word_widget: Label<'a>,
    r_cap_widget: Label<'a>,
    r_word_widget: Label<'a>,

    priority: Priority,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font_cap: Rc<Font<'_, '_>> = context.get_font(context.layout.priority_l_cap.font_size);
        let font_word: Rc<Font<'_, '_>> =
            context.get_font(context.layout.priority_l_text.font_size);

        Self {
            l_cap_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font_cap.clone(),
                context.layout.priority_l_cap,
                context.logger,
            ),
            l_word_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font_word.clone(),
                context.layout.priority_l_text,
                context.logger,
            ),
            r_cap_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font_cap.clone(),
                context.layout.priority_r_cap,
                context.logger,
            ),
            r_word_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font_word.clone(),
                context.layout.priority_r_text,
                context.logger,
            ),

            priority: Priority::None,
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.priority != data.priority {
            self.priority = data.priority;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated {
            let (left_cap_color, left_word_color) = match self.priority {
                Priority::Left => (colors::PRIORITY_RED, colors::PRIORITY_TEXT_LIGHT),
                _ => (colors::PRIORITY_DARK_RED, colors::PRIORITY_TEXT_DARK),
            };
            self.l_cap_widget.render("P", left_cap_color);
            self.l_word_widget.render("riority", left_word_color);

            let (right_cap_color, right_word_color) = match self.priority {
                Priority::Right => (colors::PRIORITY_GREEN, colors::PRIORITY_TEXT_LIGHT),
                _ => (colors::PRIORITY_DARK_GREEN, colors::PRIORITY_TEXT_DARK),
            };
            self.r_cap_widget.render("P", right_cap_color);
            self.r_word_widget.render("riority", right_word_color);
            self.updated = false;
        }
        self.l_cap_widget.draw();
        self.l_word_widget.draw();
        self.r_cap_widget.draw();
        self.r_word_widget.draw();
    }
}
