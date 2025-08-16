use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::{MatchInfo, Weapon};
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::{Label, LabelTextureCache};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    passive_counter_0_widget: Label<'a>,
    passive_counter_1_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    passive_counter: u32,
    enabled: bool,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.passive_counter_dec.font_size);

        Self {
            passive_counter_0_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.passive_counter_dec,
                context.logger,
            ),
            passive_counter_1_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.passive_counter_sec,
                context.logger,
            ),
            texture_cache: LabelTextureCache::new(),
            passive_counter: 60,
            enabled: false,
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        let enabled: bool = data.weapon != Weapon::Sabre;
        let passive_counter: u32 = data.passive_timer.get_counter();

        if self.passive_counter != passive_counter || self.enabled != enabled {
            self.passive_counter = passive_counter;
            self.enabled = enabled;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated {
            let passive_counter_text: String = if self.enabled {
                format!("{}{}", self.passive_counter / 10, self.passive_counter % 10)
            } else {
                format!("60")
            };

            let color: sdl2::pixels::Color = if self.enabled {
                colors::PASSIVE_TEXT_LIGHT
            } else {
                colors::PASSIVE_TEXT_DARK
            };

            self.passive_counter_0_widget.render(
                passive_counter_text[0..1].to_string(),
                color,
                Some(&mut self.texture_cache),
            );
            self.passive_counter_1_widget.render(
                passive_counter_text[1..2].to_string(),
                color,
                Some(&mut self.texture_cache),
            );
            self.updated = false;
        }

        self.passive_counter_0_widget.draw();
        self.passive_counter_1_widget.draw();
    }
}
