use sdl2;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::layout_structure::TextProperties;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    static_widget: Label<'a>,
}

impl<'a> Drawer<'a> {
    pub fn new(
        context: WidgetContext<'a>,
        layout: TextProperties,
        text: String,
        color: Color,
    ) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(layout.font_size);

        let mut widget: Label<'_> = Label::new(
            context.canvas.clone(),
            context.texture_creator,
            font.clone(),
            layout,
            context.logger,
        );

        widget.render(text, color, None);

        Self {
            static_widget: widget,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, _data: &MatchInfo) {}

    fn render(&mut self) {
        self.static_widget.draw();
    }
}
