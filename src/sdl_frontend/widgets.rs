use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::{self, surface};
use std::cell::RefCell;
use std::cmp::min;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::sdl_frontend::widgets;
use crate::{colors, layout_structure};

fn draw_rounded_rectangle<'a>(
    color: sdl2::pixels::Color,
    width: u32,
    height: u32,
    radius: u32,
) -> sdl2::surface::Surface<'a> {
    let mut surface: Surface<'a> =
        sdl2::surface::Surface::new(width, height, sdl2::pixels::PixelFormatEnum::RGBA8888)
            .unwrap();

    surface.fill_rect(
        sdl2::rect::Rect::new(radius as i32, 0, width - radius * 2, height),
        color,
    );
    surface.fill_rect(
        sdl2::rect::Rect::new(0, radius as i32, width, height - radius * 2),
        color,
    );

    surface
}

pub struct Label<'a> {
    canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: Rc<sdl2::ttf::Font<'a, 'a>>,
    texture: sdl2::render::Texture<'a>,

    x: i32,
    y: i32,
    width: u32,
    height: u32,

    logger: &'a crate::virtuoso_logger::Logger,
}

impl<'a> Label<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font: Rc<sdl2::ttf::Font<'a, 'a>>,

        position: layout_structure::TextProperties,

        logger: &'a crate::virtuoso_logger::Logger,
    ) -> Self {
        Self {
            canvas,
            texture_creator,
            font,
            texture: texture_creator
                .create_texture(
                    sdl2::pixels::PixelFormatEnum::RGB888,
                    sdl2::render::TextureAccess::Static,
                    1,
                    1,
                )
                .unwrap(),
            x: position.x + position.width as i32 / 2,
            y: position.y + position.height as i32 / 2,
            width: 0,
            height: 0,

            logger,
        }
    }

    pub fn render(&mut self, text: &str, color: sdl2::pixels::Color) {
        let text_surface: sdl2::surface::Surface<'a> =
            self.font.render(text).blended(color).unwrap();

        self.texture = self
            .texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap();

        self.width = text_surface.width();
        self.height = text_surface.height();
    }

    pub fn draw(&mut self) {
        let target_rect: sdl2::rect::Rect = sdl2::rect::Rect::new(
            self.x - self.width as i32 / 2,
            self.y as i32 - self.height as i32 / 2,
            self.width,
            self.height,
        );

        self.canvas
            .borrow_mut()
            .copy(&self.texture, None, Some(target_rect))
            .unwrap();
    }
}

pub struct Card<'a> {
    canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: Rc<sdl2::ttf::Font<'a, 'a>>,
    texture: sdl2::render::Texture<'a>,
    width: u32,
    height: u32,

    text_x: i32,
    text_y: i32,

    rect_x: i32,
    rect_y: i32,
    rect_width: u32,
    rect_height: u32,
    rect_radius: u32,

    logger: &'a crate::virtuoso_logger::Logger,
}

impl<'a> Card<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font: Rc<sdl2::ttf::Font<'a, 'a>>,

        text_position: layout_structure::TextProperties,
        rect_position: layout_structure::RectangleProperties,

        logger: &'a crate::virtuoso_logger::Logger,
    ) -> Self {
        Self {
            canvas,
            texture_creator,
            font,
            texture: texture_creator
                .create_texture(
                    sdl2::pixels::PixelFormatEnum::RGB888,
                    sdl2::render::TextureAccess::Static,
                    1,
                    1,
                )
                .unwrap(),

            text_x: text_position.x + text_position.width as i32 / 2,
            text_y: text_position.y + text_position.height as i32 / 2,
            width: 0,
            height: 0,

            rect_x: rect_position.x,
            rect_y: rect_position.x,
            rect_width: rect_position.width,
            rect_height: rect_position.height,
            rect_radius: rect_position.radius,

            logger,
        }
    }

    pub fn render(
        &mut self,
        text: &str,
        border_width: u32,
        card_color: sdl2::pixels::Color,
        border_color: sdl2::pixels::Color,
        text_color: sdl2::pixels::Color,
    ) {
        let radius: u32 = min(
            self.rect_radius,
            min(self.rect_height / 2, self.rect_width / 2),
        );

        let mut outer_card: Surface<'_> =
            draw_rounded_rectangle(border_color, self.rect_width, self.rect_height, radius);
        let inner_card: Surface<'_> = draw_rounded_rectangle(
            card_color,
            self.rect_width - border_width * 2,
            self.rect_height - border_width * 2,
            radius,
        );

        inner_card.blit(
            None,
            &mut outer_card,
            Some(sdl2::rect::Rect::new(
                border_width as i32,
                border_width as i32,
                self.rect_width - border_width * 2,
                self.rect_width - border_width * 2,
            )),
        );

        let text_surface: sdl2::surface::Surface<'a> =
            self.font.render(text).blended(text_color).unwrap();

        let width: u32 = outer_card.width();
        let height: u32 = outer_card.height();

        text_surface.blit(
            None,
            &mut outer_card,
            Rect::new(
                (width as i32 - text_surface.width() as i32) / 2,
                (height as i32 - text_surface.height() as i32) / 2,
                text_surface.width(),
                text_surface.height(),
            ),
        );

        self.texture = self
            .texture_creator
            .create_texture_from_surface(&outer_card)
            .unwrap();

        self.width = width;
        self.height = height;
    }

    pub fn draw(&mut self) {
        let target_rect: sdl2::rect::Rect = sdl2::rect::Rect::new(
            self.text_x - self.width as i32 / 2,
            self.text_y - self.height as i32 / 2,
            self.width,
            self.height,
        );

        self.canvas
            .borrow_mut()
            .copy(&self.texture, None, Some(target_rect))
            .unwrap();
    }
}
