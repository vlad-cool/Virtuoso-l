use sdl2;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::match_info::WarningCard;
use crate::sdl_frontend::widgets::Card;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

fn parse_caution_card<'a>(card: WarningCard) -> (&'a str, u32, Color, Color, Color) {
    if card != WarningCard::None {
        (
            "caution",
            0,
            colors::PASSIVE_YELLOW,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_LIGHT,
        )
    } else {
        (
            "caution",
            0,
            colors::PASSIVE_DARK_YELLOW,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_DARK,
        )
    }
}

fn parse_penalty_card<'a>(card: WarningCard) -> (String, u32, Color, Color, Color) {
    match card {
        WarningCard::None | WarningCard::Yellow(_) => (
            "penalty".to_string(),
            0,
            colors::PASSIVE_DARK_RED,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_DARK,
        ),
        WarningCard::Red(1) => (
            "penalty".to_string(),
            0,
            colors::PASSIVE_RED,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_LIGHT,
        ),
        WarningCard::Red(n) => (
            format!("penalty x {}", n),
            0,
            colors::PASSIVE_RED,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_LIGHT,
        ),
        WarningCard::Black(1) => (
            "penalty".to_string(),
            1,
            colors::PCARD_BLACK,
            colors::PCARD_BLACK_FRAME,
            colors::PASSIVE_TEXT_LIGHT,
        ),
        WarningCard::Black(n) => (
            format!("penalty x {}", n),
            1,
            colors::PCARD_BLACK,
            colors::PCARD_BLACK_FRAME,
            colors::PASSIVE_TEXT_LIGHT,
        ),
    }
}

pub struct Drawer<'a> {
    card_l_caution_widget: Card<'a>,
    card_l_penalty_widget: Card<'a>,
    card_r_caution_widget: Card<'a>,
    card_r_penalty_widget: Card<'a>,

    cards_l: WarningCard,
    cards_r: WarningCard,
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
            .load_font_from_rwops(rwops, layout.caution_l_text.font_size as u16)
            .unwrap_with_logger(logger);
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            card_l_caution_widget: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.caution_l_text,
                layout.caution_l_rect,
                logger,
            ),
            card_l_penalty_widget: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.penalty_l_text,
                layout.penalty_l_rect,
                logger,
            ),
            card_r_caution_widget: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.caution_r_text,
                layout.caution_r_rect,
                logger,
            ),
            card_r_penalty_widget: Card::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.penalty_r_text,
                layout.penalty_r_rect,
                logger,
            ),

            cards_l: WarningCard::Yellow(1),
            cards_r: WarningCard::Yellow(1),
        };

        res.render(WarningCard::None, WarningCard::None);
        res.draw();

        res
    }

    pub fn render(&mut self, cards_l: WarningCard, cards_r: WarningCard) {
        if self.cards_l != cards_l {
            self.cards_l = cards_l;

            let caution_card_parameters: (&'a str, u32, Color, Color, Color) =
                parse_caution_card(cards_l);
            self.card_l_caution_widget.render(
                caution_card_parameters.0,
                caution_card_parameters.1,
                caution_card_parameters.2,
                caution_card_parameters.3,
                caution_card_parameters.4,
            );

            let penalty_card_parameters: (String, u32, Color, Color, Color) =
                parse_penalty_card(cards_l);
            self.card_l_penalty_widget.render(
                penalty_card_parameters.0.as_str(),
                penalty_card_parameters.1,
                penalty_card_parameters.2,
                penalty_card_parameters.3,
                penalty_card_parameters.4,
            );
        }

        if self.cards_r != cards_r {
            self.cards_r = cards_r;

            let caution_card_parameters: (&'a str, u32, Color, Color, Color) =
                parse_caution_card(cards_r);
            self.card_r_caution_widget.render(
                caution_card_parameters.0,
                caution_card_parameters.1,
                caution_card_parameters.2,
                caution_card_parameters.3,
                caution_card_parameters.4,
            );

            let penalty_card_parameters: (String, u32, Color, Color, Color) =
                parse_penalty_card(cards_r);
            self.card_r_penalty_widget.render(
                penalty_card_parameters.0.as_str(),
                penalty_card_parameters.1,
                penalty_card_parameters.2,
                penalty_card_parameters.3,
                penalty_card_parameters.4,
            );
        }
    }

    pub fn draw(&mut self) {
        self.card_l_caution_widget.draw();
        self.card_l_penalty_widget.draw();
        self.card_r_caution_widget.draw();
        self.card_r_penalty_widget.draw();
    }
}
