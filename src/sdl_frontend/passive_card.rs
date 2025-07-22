use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::match_info::PassiveCard;
use crate::sdl_frontend::widgets::Card;
use crate::sdl_frontend::{passive_card, score};

pub struct Drawer<'a> {
    card_l_caution_widget: Card<'a>,
    card_l_penalty_widget: Card<'a>,
    card_r_caution_widget: Card<'a>,
    card_r_penalty_widget: Card<'a>,

    cards_l: PassiveCard,
    cards_r: PassiveCard,

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
            .load_font_from_rwops(rwops, layout.passive_l_top_text.font_size as u16)
            .unwrap();
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            card_l_caution_widget: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_l_bot_text,
                layout.passive_l_bot_rect,
                logger,
            ),
            card_l_penalty_widget: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_l_top_text,
                layout.passive_l_top_rect,
                logger,
            ),
            card_r_caution_widget: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_r_bot_text,
                layout.passive_r_bot_rect,
                logger,
            ),
            card_r_penalty_widget: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.passive_r_top_text,
                layout.passive_r_top_rect,
                logger,
            ),

            cards_l: PassiveCard::Yellow(1),
            cards_r: PassiveCard::Yellow(1),
            logger,
        };

        res.render(PassiveCard::None, PassiveCard::None);
        res.draw();

        res
    }

    pub fn render(&mut self, cards_l: PassiveCard, cards_r: PassiveCard) {
        if self.cards_l != cards_l {
            self.cards_l = cards_l;

            if cards_l != PassiveCard::None {
                self.card_l_caution_widget.render(
                    "P",
                    0,
                    colors::PASSIVE_YELLOW,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_LIGHT,
                );
            } else {
                self.card_l_caution_widget.render(
                    "P",
                    0,
                    colors::PASSIVE_DARK_YELLOW,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_DARK,
                );
            }

            match cards_l {
                PassiveCard::None | PassiveCard::Yellow(_) => {
                    self.card_l_penalty_widget.render(
                        "P",
                        0,
                        colors::PASSIVE_DARK_RED,
                        colors::BACKGROUND,
                        colors::PASSIVE_TEXT_DARK,
                    );
                }
                PassiveCard::Red(1) => {
                    self.card_l_penalty_widget.render(
                        "P",
                        0,
                        colors::PASSIVE_RED,
                        colors::BACKGROUND,
                        colors::PASSIVE_TEXT_LIGHT,
                    );
                }
                PassiveCard::Red(n) => {
                    self.card_l_penalty_widget.render(
                        format!("P x {}", n).as_str(),
                        0,
                        colors::PASSIVE_RED,
                        colors::BACKGROUND,
                        colors::PASSIVE_TEXT_LIGHT,
                    );
                }
                PassiveCard::Black(1) => {
                    self.card_l_penalty_widget.render(
                        "P",
                        1,
                        colors::PCARD_BLACK,
                        colors::PCARD_BLACK_FRAME,
                        colors::PASSIVE_TEXT_LIGHT,
                    );
                }
                PassiveCard::Black(n) => {
                    self.card_l_penalty_widget.render(
                        format!("P x {}", n).as_str(),
                        1,
                        colors::PCARD_BLACK,
                        colors::PCARD_BLACK_FRAME,
                        colors::PASSIVE_TEXT_LIGHT,
                    );
                }
            }
        }

        if self.cards_r != cards_r {
            self.cards_r = cards_r;

            if cards_r != PassiveCard::None {
                self.card_r_caution_widget.render(
                    "Pcard",
                    0,
                    colors::PASSIVE_YELLOW,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_LIGHT,
                );
            } else {
                self.card_r_caution_widget.render(
                    "P",
                    0,
                    colors::PASSIVE_DARK_YELLOW,
                    colors::BACKGROUND,
                    colors::PASSIVE_TEXT_DARK,
                );
            }

            match cards_r {
                PassiveCard::None | PassiveCard::Yellow(_) => {
                    self.card_r_penalty_widget.render(
                        "P",
                        0,
                        colors::PASSIVE_DARK_RED,
                        colors::BACKGROUND,
                        colors::PASSIVE_TEXT_DARK,
                    );
                }
                PassiveCard::Red(1) => {
                    self.card_r_penalty_widget.render(
                        "P",
                        0,
                        colors::PASSIVE_RED,
                        colors::BACKGROUND,
                        colors::PASSIVE_TEXT_LIGHT,
                    );
                }
                PassiveCard::Red(n) => {
                    self.card_r_penalty_widget.render(
                        format!("P x {}", n).as_str(),
                        0,
                        colors::PASSIVE_RED,
                        colors::BACKGROUND,
                        colors::PASSIVE_TEXT_LIGHT,
                    );
                }
                PassiveCard::Black(1) => {
                    self.card_r_penalty_widget.render(
                        "P",
                        2,
                        colors::PCARD_BLACK,
                        colors::PCARD_BLACK_FRAME,
                        colors::PASSIVE_TEXT_LIGHT,
                    );
                }
                PassiveCard::Black(n) => {
                    self.card_r_penalty_widget.render(
                        format!("P x {}", n).as_str(),
                        2,
                        colors::PCARD_BLACK,
                        colors::PCARD_BLACK_FRAME,
                        colors::PASSIVE_TEXT_LIGHT,
                    );
                }
            }
        }
    }

    pub fn draw(&mut self) {
        self.card_l_caution_widget.draw();
        self.card_l_penalty_widget.draw();
        self.card_r_caution_widget.draw();
        self.card_r_penalty_widget.draw();
    }
}
