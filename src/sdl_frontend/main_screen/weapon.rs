use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::match_info::{MatchInfo, Weapon};
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::{Label, LabelTextureCache};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    epee_widget: Label<'a>,
    sabre_widget: Label<'a>,
    fleuret_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    disable_inactive_weapon: bool,

    weapon: Weapon,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.epee.font_size);

        Self {
            epee_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.epee,
                context.logger,
            ),
            sabre_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.sabre,
                context.logger,
            ),
            fleuret_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.fleuret,
                context.logger,
            ),

            disable_inactive_weapon: context.layout.disable_inactive_weapon,

            texture_cache: LabelTextureCache::new(),

            updated: true,
            weapon: Weapon::Epee,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.weapon != data.weapon {
            self.weapon = data.weapon;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated {
            self.epee_widget.render(
                "epee".to_string(),
                if self.weapon == Weapon::Epee {
                    colors::WEAPON_TEXT_LIGHT
                } else {
                    colors::WEAPON_TEXT_DARK
                },
                Some(&mut self.texture_cache),
            );

            self.sabre_widget.render(
                "sabre".to_string(),
                if self.weapon == Weapon::Sabre {
                    colors::WEAPON_TEXT_LIGHT
                } else {
                    colors::WEAPON_TEXT_DARK
                },
                Some(&mut self.texture_cache),
            );

            self.fleuret_widget.render(
                "fleuret".to_string(),
                if self.weapon == Weapon::Fleuret {
                    colors::WEAPON_TEXT_LIGHT
                } else {
                    colors::WEAPON_TEXT_DARK
                },
                Some(&mut self.texture_cache),
            );
            self.updated = false;
        }

        if self.disable_inactive_weapon {
            match self.weapon {
                Weapon::Epee => self.epee_widget.draw(),
                Weapon::Sabre => self.sabre_widget.draw(),
                Weapon::Fleuret => self.fleuret_widget.draw(),
            }
        } else {
            self.epee_widget.draw();
            self.sabre_widget.draw();
            self.fleuret_widget.draw();
        }
    }
}
