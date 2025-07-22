use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::colors;
use crate::layout_structure::Layout;
use crate::match_info::Weapon;
use crate::sdl_frontend::widgets::Label;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

pub struct Drawer<'a> {
    epee_widget: Label<'a>,
    sabre_widget: Label<'a>,
    fleuret_widget: Label<'a>,

    weapon: Weapon,
}

impl<'a> Drawer<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
        rwops: sdl2::rwops::RWops<'a>,
        layout: &Layout,

        logger: &'a Logger,
    ) -> Self {
        let font: sdl2::ttf::Font<'a, 'a> = ttf_context
            .load_font_from_rwops(rwops, layout.epee.font_size as u16)
            .unwrap_with_logger(logger);
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            epee_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.epee,
                logger,
            ),
            sabre_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.sabre,
                logger,
            ),
            fleuret_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.fleuret,
                logger,
            ),

            weapon: Weapon::Sabre,
        };

        res.render(Weapon::Epee);
        res.draw();

        res
    }

    pub fn render(&mut self, weapon: Weapon) {
        if self.weapon != weapon {
            self.weapon = weapon;

            self.epee_widget.render(
                "epee",
                if weapon == Weapon::Epee {
                    colors::WEAPON_TEXT_LIGHT
                } else {
                    colors::WEAPON_TEXT_DARK
                },
            );

            self.sabre_widget.render(
                "sabre",
                if weapon == Weapon::Sabre {
                    colors::WEAPON_TEXT_LIGHT
                } else {
                    colors::WEAPON_TEXT_DARK
                },
            );

            self.fleuret_widget.render(
                "fleuret",
                if weapon == Weapon::Fleuret {
                    colors::WEAPON_TEXT_LIGHT
                } else {
                    colors::WEAPON_TEXT_DARK
                },
            );
        }
    }

    pub fn draw(&mut self) {
        self.epee_widget.draw();
        self.sabre_widget.draw();
        self.fleuret_widget.draw();
    }
}
