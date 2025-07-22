use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors::{self, PRIORITY_TEXT_DARK, PRIORITY_TEXT_LIGHT};
use crate::match_info::Priority;
use crate::sdl_frontend::widgets::Label;
use crate::sdl_frontend::{priority, score};

pub struct Drawer<'a> {
    l_cap_widget: Label<'a>,
    l_word_widget: Label<'a>,
    r_cap_widget: Label<'a>,
    r_word_widget: Label<'a>,

    priority: Priority,

    logger: &'a crate::virtuoso_logger::Logger,
}

impl<'a> Drawer<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        rwops_0: sdl2::rwops::RWops<'a>,
        rwops_1: sdl2::rwops::RWops<'a>,
        layout: &crate::layout_structure::Layout,

        logger: &'a crate::virtuoso_logger::Logger,
    ) -> Self {
        let font_cap: sdl2::ttf::Font<'a, 'a> = ttf_context
            .load_font_from_rwops(rwops_0, layout.priority_l_cap.font_size as u16)
            .unwrap();
        let font_cap: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font_cap);

        let font_word: sdl2::ttf::Font<'a, 'a> = ttf_context
            .load_font_from_rwops(rwops_1, layout.priority_l_text.font_size as u16)
            .unwrap();
        let font_word: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font_word);

        let mut res: Drawer<'a> = Self {
            l_cap_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font_cap.clone(),
                layout.priority_l_cap,
                logger,
            ),
            l_word_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font_word.clone(),
                layout.priority_l_text,
                logger,
            ),
            r_cap_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font_cap.clone(),
                layout.priority_r_cap,
                logger,
            ),
            r_word_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font_word.clone(),
                layout.priority_r_text,
                logger,
            ),

            priority: Priority::Left,

            logger,
        };

        res.render(Priority::None);
        res.draw();

        res
    }

    pub fn render(&mut self, priority: Priority) {
        if self.priority != priority {
            self.priority = priority;

            let (left_cap_color, left_word_color) = match priority {
                Priority::Left => (colors::PRIORITY_RED, PRIORITY_TEXT_LIGHT),
                _ => (colors::PRIORITY_DARK_RED, PRIORITY_TEXT_DARK),
            };
            self.l_cap_widget.render("P", left_cap_color);
            self.l_word_widget.render("riority", left_word_color);

            let (right_cap_color, right_word_color) = match priority {
                Priority::Right => (colors::PRIORITY_GREEN, PRIORITY_TEXT_LIGHT),
                _ => (colors::PRIORITY_DARK_GREEN, PRIORITY_TEXT_DARK),
            };
            self.r_cap_widget.render("P", left_cap_color);
            self.r_word_widget.render("riority", left_word_color);
        }
    }

    pub fn draw(&mut self) {
        self.l_cap_widget.draw();
        self.l_word_widget.draw();
        self.r_cap_widget.draw();
        self.r_word_widget.draw();
    }
}
