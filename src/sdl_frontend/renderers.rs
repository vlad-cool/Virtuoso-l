use sdl2;
use std::cell::RefCell;
use std::rc::Rc;

use crate::layout_structure;

pub struct TextRenderer<'a> {
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

impl<'a> TextRenderer<'a> {
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
