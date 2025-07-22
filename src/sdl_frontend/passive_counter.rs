use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::sdl_frontend::widgets::Label;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

pub struct Drawer<'a> {
    passive_counter_0_widget: Label<'a>,
    passive_counter_1_widget: Label<'a>,

    passive_counter: u32,
    enabled: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        rwops: sdl2::rwops::RWops<'a>,
        layout: &crate::layout_structure::Layout,

        logger: &'a Logger,
    ) -> Self {
        let font: sdl2::ttf::Font<'a, 'a> = ttf_context
            .load_font_from_rwops(rwops, layout.passive_counter_dec.font_size as u16)
            .unwrap_with_logger(logger);
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            passive_counter_0_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_counter_dec,
                logger,
            ),
            passive_counter_1_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_counter_sec,
                logger,
            ),
            passive_counter: 1,
            enabled: false,
        };

        res.render(60, true);
        res.draw();

        res
    }

    pub fn render(&mut self, passive_counter: u32, enabled: bool) {
        if self.passive_counter != passive_counter || self.enabled != enabled {
            self.passive_counter = passive_counter;
            self.enabled = enabled;

            let passive_counter_text: String = if enabled {
                format!("{}{}", passive_counter / 10, passive_counter % 10)
            } else {
                format!("  ")
            };

            let color: sdl2::pixels::Color = if enabled {
                colors::PASSIVE_TEXT_LIGHT
            } else {
                colors::PASSIVE_TEXT_DARK
            };

            self.passive_counter_0_widget
                .render(&passive_counter_text[0..1], color);
            self.passive_counter_1_widget
                .render(&passive_counter_text[1..2], color);
        }
    }

    pub fn draw(&mut self) {
        self.passive_counter_0_widget.draw();
        self.passive_counter_1_widget.draw();
    }
}
