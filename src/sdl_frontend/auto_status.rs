use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::sdl_frontend::score;
use crate::sdl_frontend::widgets::Label;

pub struct Drawer<'a> {
    timer_widget: Label<'a>,
    score_widget: Label<'a>,

    auto_timer_on: bool,
    auto_score_on: bool,

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
            .load_font_from_rwops(rwops, layout.auto_timer_status.font_size as u16)
            .unwrap();
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            score_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.auto_score_status,
                logger,
            ),
            timer_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.auto_timer_status,
                logger,
            ),

            auto_score_on: true,
            auto_timer_on: true,

            logger,
        };

        res.render(false, false);
        res.draw();

        res
    }

    pub fn render(&mut self, auto_timer_on: bool, auto_score_on: bool) {
        if self.auto_timer_on != auto_timer_on {
            self.auto_timer_on = auto_timer_on;

            let (text, color) = if auto_timer_on {
                ("auto timer\non", colors::AUTO_STATUS_TEXT_LIGHT)
            } else {
                ("auto timer\noff", colors::AUTO_STATUS_TEXT_DARK)
            };

            self.timer_widget.render(text, color);
        }
        
        if self.auto_score_on != auto_score_on {
            self.auto_score_on = auto_score_on;

            let (text, color) = if auto_score_on {
                ("auto score\non", colors::AUTO_STATUS_TEXT_LIGHT)
            } else {
                ("auto score\noff", colors::AUTO_STATUS_TEXT_DARK)
            };

            self.score_widget.render(text, color);
        }
    }

    pub fn draw(&mut self) {
        self.timer_widget.draw();
        self.score_widget.draw();
    }
}
