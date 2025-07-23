use sdl2;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::colors;
use crate::match_info::{MatchInfo, PassiveCard};
use crate::sdl_frontend::widgets::Card;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

fn parse_caution_card<'a>(card: PassiveCard) -> (&'a str, u32, Color, Color, Color) {
    if card != PassiveCard::None {
        (
            "P",
            0,
            colors::PASSIVE_YELLOW,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_LIGHT,
        )
    } else {
        (
            "P",
            0,
            colors::PASSIVE_DARK_YELLOW,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_DARK,
        )
    }
}

fn parse_penalty_card<'a>(card: PassiveCard) -> (String, u32, Color, Color, Color) {
    match card {
        PassiveCard::None | PassiveCard::Yellow(_) => (
            "P".to_string(),
            0,
            colors::PASSIVE_DARK_RED,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_DARK,
        ),
        PassiveCard::Red(1) => (
            "P".to_string(),
            0,
            colors::PASSIVE_RED,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_LIGHT,
        ),
        PassiveCard::Red(n) => (
            format!("P x {}", n),
            0,
            colors::PASSIVE_RED,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_LIGHT,
        ),
        PassiveCard::Black(1) => (
            "P".to_string(),
            1,
            colors::PCARD_BLACK,
            colors::PCARD_BLACK_FRAME,
            colors::PASSIVE_TEXT_LIGHT,
        ),
        PassiveCard::Black(n) => (
            format!("P x {}", n),
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

    cards_l: PassiveCard,
    cards_l_updated: bool,
    cards_r: PassiveCard,
    cards_r_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> =
            context.get_font(context.layout.passive_l_top_text.font_size as u16);

        Self {
            card_l_caution_widget: Card::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.passive_l_bot_text,
                context.layout.passive_l_bot_rect,
                context.logger,
            ),
            card_l_penalty_widget: Card::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.passive_l_top_text,
                context.layout.passive_l_top_rect,
                context.logger,
            ),
            card_r_caution_widget: Card::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.passive_r_bot_text,
                context.layout.passive_r_bot_rect,
                context.logger,
            ),
            card_r_penalty_widget: Card::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.passive_r_top_text,
                context.layout.passive_r_top_rect,
                context.logger,
            ),

            cards_l: PassiveCard::None,
            cards_l_updated: true,
            cards_r: PassiveCard::None,
            cards_r_updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.cards_l != data.left_fencer.passive_card {
            self.cards_l = data.left_fencer.passive_card;
            self.cards_l_updated = true;
        }
        if self.cards_r != data.right_fencer.passive_card {
            self.cards_r = data.right_fencer.passive_card;
            self.cards_r_updated = true;
        }
    }

    fn render(&mut self) {
        if self.cards_l_updated {
            let (text, border_width, card_color, border_color, text_color) =
                parse_caution_card(self.cards_l);
            self.card_l_caution_widget.render(
                text,
                border_width,
                card_color,
                border_color,
                text_color,
            );

            let (text, border_width, card_color, border_color, text_color) =
                parse_penalty_card(self.cards_l);
            self.card_l_penalty_widget.render(
                text.as_str(),
                border_width,
                card_color,
                border_color,
                text_color,
            );
            self.cards_l_updated = false;
        }

        if self.cards_r_updated {
            let (text, border_width, card_color, border_color, text_color) =
                parse_caution_card(self.cards_r);
            self.card_r_caution_widget.render(
                text,
                border_width,
                card_color,
                border_color,
                text_color,
            );

            let (text, border_width, card_color, border_color, text_color) =
                parse_penalty_card(self.cards_r);
            self.card_r_penalty_widget.render(
                text.as_str(),
                border_width,
                card_color,
                border_color,
                text_color,
            );
            self.cards_r_updated = false;
        }

        self.card_l_caution_widget.draw();
        self.card_l_penalty_widget.draw();
        self.card_r_caution_widget.draw();
        self.card_r_penalty_widget.draw();
    }
}
