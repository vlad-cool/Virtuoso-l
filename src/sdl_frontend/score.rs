use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::sdl_frontend::score;
use crate::sdl_frontend::widgets::Label;

pub struct Drawer<'a> {
    score_l_l_renderer: Label<'a>,
    score_l_r_renderer: Label<'a>,
    score_r_l_renderer: Label<'a>,
    score_r_r_renderer: Label<'a>,

    score_l: u32,
    score_r: u32,

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
            .load_font_from_rwops(rwops, layout.score_l_l.font_size as u16)
            .unwrap();
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            score_l_l_renderer: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.score_l_l,
                logger,
            ),
            score_l_r_renderer: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.score_l_r,
                logger,
            ),
            score_r_l_renderer: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.score_r_l,
                logger,
            ),
            score_r_r_renderer: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.score_r_r,
                logger,
            ),
            score_l: 1,
            score_r: 1,
            logger,
        };

        res.render(0, 0);
        res.draw();

        res
    }

    pub fn render(&mut self, score_l: u32, score_r: u32) {
        if self.score_l != score_l {
            self.score_l = score_l;

            let score_l_l_text: String = if score_l < 10 {
                format!("{}", score_l)
            } else {
                format!("{}", score_l / 10)
            };
            self.score_l_l_renderer
                .render(score_l_l_text.as_str(), colors::SCORE_LEFT);

            let score_l_r_text: String = if score_l < 10 {
                " ".to_string()
            } else {
                format!("{}", score_l % 10)
            };
            self.score_l_r_renderer
                .render(score_l_r_text.as_str(), colors::SCORE_LEFT);
        }

        if self.score_r != score_r {
            self.score_r = score_r;
            let score_r_l_text: String = if score_r < 10 {
                " ".to_string()
            } else {
                format!("{}", score_r / 10)
            };
            self.score_r_l_renderer
                .render(score_r_l_text.as_str(), colors::SCORE_RIGHT);

            let score_r_r_text: String = if score_r < 10 {
                format!("{}", score_r)
            } else {
                format!("{}", score_r % 10)
            };
            self.score_r_r_renderer
                .render(score_r_r_text.as_str(), colors::SCORE_RIGHT);
        }
    }

    pub fn draw(&mut self) {
        self.score_l_l_renderer.draw();
        self.score_l_r_renderer.draw();
        self.score_r_l_renderer.draw();
        self.score_r_r_renderer.draw();
    }
}
