use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::sdl_frontend::score;
use crate::sdl_frontend::widgets::Card;

pub struct Drawer<'a> {
    card_l_caution: Card<'a>,
    card_l_penalty: Card<'a>,
    card_r_caution: Card<'a>,
    card_r_penalty: Card<'a>,

    cards_l: u32,
    cards_r: u32,

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
            .load_font_from_rwops(rwops, layout.caution_l_text.font_size as u16)
            .unwrap();
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            card_l_caution: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_l_bot_text,
                layout.passive_l_bot_rect,
                logger,
            ),
            card_l_penalty: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_l_top_text,
                layout.passive_l_top_rect,
                logger,
            ),
            card_r_caution: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_r_bot_text,
                layout.passive_r_bot_rect,
                logger,
            ),
            card_r_penalty: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_r_top_text,
                layout.passive_r_top_rect,
                logger,
            ),

            cards_l: 1,
            cards_r: 1,
            logger,
        };

        res.render(0, 0);
        res.draw();

        res
    }

    pub fn render(&mut self, cards_l: u32, cards_r: u32) {
        if self.cards_l != cards_l {
            self.cards_l = cards_l;

            if cards_l > 0 {
                self.card_l_caution.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_YELLOW,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_LIGHT,
                );
            } else {
                self.card_l_caution.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_DARK_YELLOW,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_DARK,
                );
            }

            if cards_l > 1 {
                self.card_l_penalty.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_RED,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_LIGHT,
                );
            } else {
                self.card_l_penalty.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_DARK_RED,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_DARK,
                );
            }
        }
        
        if self.cards_r != cards_r {
            self.cards_r = cards_r;

            if cards_r > 0 {
                self.card_r_caution.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_YELLOW,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_LIGHT,
                );
            } else {
                self.card_r_caution.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_DARK_YELLOW,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_DARK,
                );
            }

            if cards_r > 1 {
                self.card_r_penalty.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_RED,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_LIGHT,
                );
            } else {
                self.card_r_penalty.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_DARK_RED,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_DARK,
                );
            }
        }
    }

    pub fn draw(&mut self) {
        self.card_l_caution.draw();
        self.card_l_penalty.draw();
        self.card_r_caution.draw();
        self.card_r_penalty.draw();
    }
}
