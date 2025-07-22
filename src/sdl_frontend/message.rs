use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::sdl_frontend::score;
use crate::sdl_frontend::widgets::Label;

pub struct Drawer<'a> {
    message_widget: Label<'a>,

    message: String,

    logger: &'a crate::virtuoso_logger::Logger,
}

impl<'a> Drawer<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        rwops: sdl2::rwops::RWops<'a>,
        layout: &crate::layout_structure::Layout,

        logger: &'a crate::virtuoso_logger::Logger,
    ) -> Self {
        let font: sdl2::ttf::Font<'a, 'a> = ttf_context
            .load_font_from_rwops(rwops, layout.timer_text.font_size as u16)
            .unwrap();
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            message_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.timer_text,
                logger,
            ),
            message: "".to_string(),
            logger,
        };

        res.render(" ".to_string());
        res.draw();

        res
    }

    pub fn render(&mut self, message: String) {
        if self.message != message {
            self.message = message.clone();

            self.message_widget
                .render(message.as_str(), colors::TIMER_ORANGE);
        }
    }

    pub fn draw(&mut self) {
        self.message_widget.draw();
    }
}
