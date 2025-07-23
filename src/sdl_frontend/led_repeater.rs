use sdl2;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::colors;
use crate::layout_structure::RectangleProperties;
use crate::sdl_frontend::widgets::Indicator;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

pub struct Drawer<'a> {
    l_color_widget: Indicator<'a>,
    l_white_widget: Indicator<'a>,
    r_color_widget: Indicator<'a>,
    r_white_widget: Indicator<'a>,

    l_color_led_on: bool,
    l_white_led_on: bool,
    r_color_led_on: bool,
    r_white_led_on: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(
        canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        layout: &crate::layout_structure::Layout,

        logger: &'a Logger,
    ) -> Self {
        let mut res: Drawer<'a> = Self {
            l_color_widget: Indicator::new(
                canvas.clone(),
                texture_creator,
                layout.left_color_indicator,
                logger,
            ),
            l_white_widget: Indicator::new(
                canvas.clone(),
                texture_creator,
                layout.left_white_indicator,
                logger,
            ),
            r_color_widget: Indicator::new(
                canvas.clone(),
                texture_creator,
                layout.right_color_indicator,
                logger,
            ),
            r_white_widget: Indicator::new(
                canvas.clone(),
                texture_creator,
                layout.right_white_indicator,
                logger,
            ),

            l_color_led_on: true,
            l_white_led_on: true,
            r_color_led_on: true,
            r_white_led_on: true,
        };

        res.render(false, false, false, false);
        res.draw();

        res
    }

    pub fn render(
        &mut self,
        l_color_led_on: bool,
        l_white_led_on: bool,
        r_color_led_on: bool,
        r_white_led_on: bool,
    ) {
        if self.l_color_led_on != l_color_led_on {
            self.l_color_led_on = l_color_led_on;

            let color: sdl2::pixels::Color = if l_color_led_on {
                colors::COLOR_LABELS_RED
            } else {
                colors::COLOR_LABELS_DARK_RED
            };

            self.l_color_widget.render(color);
        }
        if self.l_white_led_on != l_white_led_on {
            self.l_white_led_on = l_white_led_on;

            let color: sdl2::pixels::Color = if l_white_led_on {
                colors::WHITE_LABELS_LIGHT
            } else {
                colors::WHITE_LABELS_DARK
            };

            self.l_white_widget.render(color);
        }
        if self.r_color_led_on != r_color_led_on {
            self.r_color_led_on = r_color_led_on;

            let color: sdl2::pixels::Color = if r_color_led_on {
                colors::COLOR_LABELS_GREEN
            } else {
                colors::COLOR_LABELS_DARK_GREEN
            };

            self.r_color_widget.render(color);
        }
        if self.r_white_led_on != r_white_led_on {
            self.r_white_led_on = r_white_led_on;

            let color: sdl2::pixels::Color = if r_white_led_on {
                colors::WHITE_LABELS_LIGHT
            } else {
                colors::WHITE_LABELS_DARK
            };

            self.r_white_widget.render(color);
        }
    }

    pub fn draw(&mut self) {
        self.l_color_widget.draw();
        self.l_white_widget.draw();
        self.r_color_widget.draw();
        self.r_white_widget.draw();
    }
}
