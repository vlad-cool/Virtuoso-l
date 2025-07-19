use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::sdl_frontend::renderers::TextRenderer;
use crate::sdl_frontend::{period, score};

pub struct Drawer<'a> {
    period_renderer: TextRenderer<'a>,

    period: u32,

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
            .load_font_from_rwops(rwops, layout.period.font_size as u16)
            .unwrap();
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            period_renderer: TextRenderer::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.period,
                logger,
            ),
            period: 0,
            logger,
        };

        res.render(1);
        res.draw();

        res
    }

    pub fn render(&mut self, period: u32) {
        if self.period != period {
            if period >= 10 {
                self.logger.error(format!("Period is {period} >= 10"));
            }

            let period: u32 = period % 10;
            self.period = period;

            let period_text: String = format!("{}", period);
            self.period_renderer
                .render(&period_text.as_str(), colors::PERIOD);
        }
    }

    pub fn draw(&mut self) {
        self.period_renderer.draw();
    }
}
