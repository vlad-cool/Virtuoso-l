use sdl2;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::{MatchInfo, WarningCard};
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::Card;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

fn parse_penalty_card<'a>(card: WarningCard) -> (String, u32, Color, Color, Color) {
    match card {
        WarningCard::None => (
            "caution".to_string(),
            0,
            colors::PASSIVE_DARK_YELLOW,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_DARK,
        ),
        WarningCard::Yellow(1) => (
            "caution".to_string(),
            0,
            colors::PASSIVE_YELLOW,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_LIGHT,
        ),
        WarningCard::Yellow(n) => (
            format!("caution x {}", n),
            0,
            colors::PASSIVE_YELLOW,
            colors::BACKGROUND,
            colors::PASSIVE_TEXT_LIGHT,
        ),
        WarningCard::Red(n) => (
            format!("pen x {}", n),
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
            format!("pen x {}", n),
            1,
            colors::PCARD_BLACK,
            colors::PCARD_BLACK_FRAME,
            colors::PASSIVE_TEXT_LIGHT,
        ),
    }
}

pub struct Drawer<'a> {
    card_l_penalty_widget: Card<'a>,
    card_r_penalty_widget: Card<'a>,

    cards_l: WarningCard,
    cards_l_updated: bool,
    cards_r: WarningCard,
    cards_r_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.caution_l_text.font_size);

        Self {
            card_l_penalty_widget: Card::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.penalty_l_text,
                context.layout.penalty_l_rect,
                context.logger,
            ),
            card_r_penalty_widget: Card::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.penalty_r_text,
                context.layout.penalty_r_rect,
                context.logger,
            ),

            cards_l: WarningCard::None,
            cards_l_updated: true,
            cards_r: WarningCard::None,
            cards_r_updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.cards_l != data.left_fencer.warning_card {
            self.cards_l = data.left_fencer.warning_card;
            self.cards_l_updated = true;
        }
        if self.cards_r != data.right_fencer.warning_card {
            self.cards_r = data.right_fencer.warning_card;
            self.cards_r_updated = true;
        }
    }

    fn render(&mut self) {
        if self.cards_l_updated {
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

        self.card_l_penalty_widget.draw();
        self.card_r_penalty_widget.draw();
    }
}
