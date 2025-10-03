use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::sdl_frontend::layout_structure;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct LabelHashKey {
    pub color: Color,
    pub text: String,
}

pub struct LabelTextureCache<'a> {
    cache: HashMap<LabelHashKey, (u32, u32, Rc<Option<Texture<'a>>>)>,
}

fn render_text<'a>(
    text: &str,
    color: Color,
    font: Rc<sdl2::ttf::Font<'a, 'a>>,
    logger: &Logger,
) -> Surface<'a> {
    let mut surfaces: std::vec::Vec<sdl2::surface::Surface<'_>> = std::vec::Vec::new();

    let mut width: u32 = 0;
    let mut height: u32 = 0;

    for line in text.split("\n") {
        let surface: sdl2::surface::Surface<'a> = font.render(line).blended(color).unwrap_or(
            sdl2::surface::Surface::new(1, 1, sdl2::pixels::PixelFormatEnum::RGBA8888)
                .unwrap_with_logger(logger),
        );

        width = std::cmp::max(width, surface.width());
        height += surface.height();
        surfaces.push(surface);
    }

    let mut text_surface: Surface<'a> = sdl2::surface::Surface::new(
        width,
        height as u32,
        sdl2::pixels::PixelFormatEnum::RGBA8888,
    )
    .unwrap_with_logger(logger);

    let mut y_pos: i32 = 0;
    for surface in surfaces {
        let dst_rect = sdl2::rect::Rect::new(
            (width - surface.width()) as i32 / 2,
            y_pos,
            surface.width(),
            surface.height(),
        );
        surface
            .blit(None, &mut text_surface, dst_rect)
            .unwrap_with_logger(logger);
        y_pos += surface.height() as i32;
    }
    text_surface
}

fn draw_label_texture<'a>(
    key: LabelHashKey,
    texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
    font: Rc<sdl2::ttf::Font<'a, 'a>>,
    logger: &Logger,
) -> (u32, u32, Rc<Option<Texture<'a>>>) {
    let text: String = key.text;

    if text == "" {
        let texture: Option<Texture<'a>> = None;
        let width: u32 = 0;
        let height: u32 = 0;
        return (width, height, Rc::new(texture));
    }

    let text_surface: Surface<'_> = render_text(text.as_str(), key.color, font, logger);

    let texture = Some(
        texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap_with_logger(logger),
    );

    let width: u32 = text_surface.width();
    let height: u32 = text_surface.height();

    (width, height, Rc::new(texture))
}

impl<'a> LabelTextureCache<'a> {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(
        &mut self,
        key: LabelHashKey,
        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
        font: Rc<sdl2::ttf::Font<'a, 'a>>,
        logger: &Logger,
    ) -> (u32, u32, Rc<Option<Texture<'a>>>) {
        if let Some(value) = self.cache.get(&key) {
            return value.clone();
        } else {
            self.cache.insert(
                key.clone(),
                draw_label_texture(key.clone(), texture_creator, font, logger),
            );

            return self.cache[&key].clone();
        }
    }
}

fn draw_rounded_rectangle<'a>(
    color: Color,
    width: u32,
    height: u32,
    radius: u32,
    logger: &Logger,
) -> sdl2::surface::Surface<'a> {
    let mut surface: Surface<'a> = sdl2::surface::Surface::new(
        max(width, 1),
        max(height, 1),
        sdl2::pixels::PixelFormatEnum::RGBA8888,
    )
    .unwrap_with_logger(logger);

    surface
        .fill_rect(
            sdl2::rect::Rect::new(radius as i32, 0, width - radius * 2, height),
            color,
        )
        .unwrap_with_logger(logger);
    surface
        .fill_rect(
            sdl2::rect::Rect::new(0, radius as i32, radius, height - radius * 2),
            color,
        )
        .unwrap_with_logger(logger);
    surface
        .fill_rect(
            sdl2::rect::Rect::new(
                width as i32 - radius as i32,
                radius as i32,
                radius,
                height - radius * 2,
            ),
            color,
        )
        .unwrap_with_logger(logger);

    if width == 0 || height == 0 {
        return surface;
    }

    let format: sdl2::pixels::PixelFormat = surface.pixel_format();

    surface.with_lock_mut(|pixels: &mut [u8]| {
        for x in 0..(radius + 1) as usize {
            for y in 0..(radius + 1) as usize {
                let radius: usize = radius as usize;
                let width: usize = width as usize;
                let height: usize = height as usize;

                if x * x + y * y <= (radius + 1) * (radius + 1) {
                    let mut color = color.to_u32(&format).to_le_bytes();

                    if x * x + y * y > radius * radius {
                        let val: f32 = 1.0
                            - (((x * x + y * y) as f32).sqrt() - ((radius * radius) as f32).sqrt());
                        color[3] = (color[3] as f32 * val) as u8;
                    }

                    let x: usize = radius - x;
                    let y: usize = radius - y;

                    for index in [
                        (x + y * width) * 4,
                        (width - x - 1 + y * width) * 4,
                        (x + (height - y - 1) * width) * 4,
                        (width - x - 1 + (height - y - 1) * width) * 4,
                    ] {
                        for i in 0..4 {
                            pixels[index + i] = color[i];
                        }
                    }
                }
            }
        }
    });

    surface
}

pub struct Label<'a> {
    canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: Rc<sdl2::ttf::Font<'a, 'a>>,
    texture: Rc<Option<sdl2::render::Texture<'a>>>,

    x: i32,
    y: i32,
    width: u32,
    height: u32,
    logger: &'a Logger,
}

impl<'a> Label<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font: Rc<sdl2::ttf::Font<'a, 'a>>,
        position: layout_structure::TextProperties,

        logger: &'a Logger,
    ) -> Self {
        Self {
            canvas,
            texture_creator,
            font,
            texture: Rc::new(None),

            x: position.x + position.width as i32 / 2,
            y: position.y + position.height as i32 / 2,
            width: 0,
            height: 0,
            logger,
        }
    }

    pub fn render(
        &mut self,
        text: String,
        color: Color,
        texture_cache: Option<&mut LabelTextureCache<'a>>,
    ) {
        let key: LabelHashKey = LabelHashKey {
            text: text.clone(),
            color,
        };

        let (width, height, texture) = if let Some(texture_cache) = texture_cache {
            texture_cache.get(key, self.texture_creator, self.font.clone(), self.logger)
        } else {
            draw_label_texture(key, self.texture_creator, self.font.clone(), self.logger)
        };

        self.width = width;
        self.height = height;
        self.texture = texture.clone();
    }

    pub fn draw(&mut self) {
        if let Some(texture) = &*self.texture {
            let target_rect: sdl2::rect::Rect = sdl2::rect::Rect::new(
                self.x - self.width as i32 / 2,
                self.y as i32 - self.height as i32 / 2,
                self.width,
                self.height,
            );

            self.canvas
                .borrow_mut()
                .copy(&texture, None, Some(target_rect))
                .unwrap_with_logger(self.logger);
        }
    }
}

pub struct Card<'a> {
    canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: Rc<sdl2::ttf::Font<'a, 'a>>,
    texture: Option<sdl2::render::Texture<'a>>,
    width: u32,
    height: u32,

    text_x: i32,
    text_y: i32,

    rect_x: i32,
    rect_y: i32,
    rect_width: u32,
    rect_height: u32,
    rect_radius: u32,

    logger: &'a Logger,
}

impl<'a> Card<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        font: Rc<sdl2::ttf::Font<'a, 'a>>,

        text_position: layout_structure::TextProperties,
        rect_position: layout_structure::RectangleProperties,

        logger: &'a Logger,
    ) -> Self {
        Self {
            canvas,
            texture_creator,
            font,
            texture: None,

            text_x: text_position.x,
            text_y: text_position.y,
            width: 0,
            height: 0,

            rect_x: rect_position.x,
            rect_y: rect_position.y,
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
        card_color: Color,
        border_color: Color,
        text_color: Color,
    ) {
        let radius: u32 = min(
            self.rect_radius,
            min(self.rect_height / 2, self.rect_width / 2),
        );

        let mut outer_card: Surface<'_> = draw_rounded_rectangle(
            border_color,
            self.rect_width,
            self.rect_height,
            radius,
            self.logger,
        );
        let inner_card: Surface<'_> = draw_rounded_rectangle(
            card_color,
            self.rect_width - border_width * 2,
            self.rect_height - border_width * 2,
            radius - border_width,
            self.logger,
        );

        inner_card
            .blit(
                None,
                &mut outer_card,
                Some(sdl2::rect::Rect::new(
                    self.text_x - self.rect_x + border_width as i32,
                    self.text_y - self.rect_y + border_width as i32,
                    self.rect_width - border_width * 2,
                    self.rect_height - border_width * 2,
                )),
            )
            .unwrap_with_logger(self.logger);

        let text_surface: Surface<'_> =
            render_text(text, text_color, self.font.clone(), self.logger);

        let width: u32 = outer_card.width();
        let height: u32 = outer_card.height();

        text_surface
            .blit(
                None,
                &mut outer_card,
                Rect::new(
                    (width as i32 - text_surface.width() as i32) / 2,
                    (height as i32 - text_surface.height() as i32) / 2,
                    text_surface.width(),
                    text_surface.height(),
                ),
            )
            .unwrap_with_logger(self.logger);

        self.texture = Some(
            self.texture_creator
                .create_texture_from_surface(&outer_card)
                .unwrap_with_logger(self.logger),
        );

        self.width = width;
        self.height = height;
    }

    pub fn draw(&mut self) {
        if let Some(texture) = &self.texture {
            let target_rect: sdl2::rect::Rect =
                sdl2::rect::Rect::new(self.rect_x, self.rect_y, self.rect_width, self.rect_height);

            self.canvas
                .borrow_mut()
                .copy(texture, None, Some(target_rect))
                .unwrap_with_logger(self.logger);
        }
    }
}

pub struct Indicator<'a> {
    canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    texture: Option<sdl2::render::Texture<'a>>,

    x: i32,
    y: i32,
    width: u32,
    height: u32,
    radius: u32,

    logger: &'a Logger,
}

impl<'a> Indicator<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,

        position: layout_structure::RectangleProperties,

        logger: &'a Logger,
    ) -> Self {
        Self {
            canvas,
            texture_creator,
            texture: None,

            x: position.x,
            y: position.y,
            width: position.width,
            height: position.height,
            radius: position.radius,

            logger,
        }
    }

    pub fn render(&mut self, color: Color) {
        if self.width == 0 || self.height == 0 {
            self.texture = None;
        } else {
            let radius: u32 = min(self.radius, min(self.height / 2, self.width / 2));

            let surface: Surface<'_> =
                draw_rounded_rectangle(color, self.width, self.height, radius, self.logger);

            self.texture = Some(
                self.texture_creator
                    .create_texture_from_surface(&surface)
                    .unwrap_with_logger(&self.logger),
            );
        }
    }

    #[allow(dead_code)]
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    #[allow(dead_code)]
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    #[allow(dead_code)]
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    #[allow(dead_code)]
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn draw(&mut self) {
        if let Some(texture) = &self.texture {
            let target_rect: sdl2::rect::Rect =
                sdl2::rect::Rect::new(self.x, self.y, self.width, self.height);

            self.canvas
                .borrow_mut()
                .copy(texture, None, Some(target_rect))
                .unwrap_with_logger(&self.logger);
        }
    }
}
