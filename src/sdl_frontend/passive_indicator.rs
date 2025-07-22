use sdl2;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::colors;
use crate::layout_structure::RectangleProperties;
use crate::sdl_frontend::widgets::Indicator;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

pub struct Drawer<'a> {
    passive_indicator_widget: Indicator<'a>,
    position: RectangleProperties,

    passive_indicator: u32,
    passive_counter: u32,
    time: Duration,
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
            .load_font_from_rwops(rwops, layout.passive_counter_dec.font_size as u16)
            .unwrap_with_logger(logger);
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            passive_indicator_widget: Indicator::new(
                canvas.clone(),
                texture_creator,
                layout.passive_indicator,
                logger,
            ),
            position: layout.passive_indicator.clone(),
            passive_indicator: 1,
            passive_counter: 1,
            time: Duration::from_secs(0),
        };

        res.render(0, 0, Duration::from_secs(0));
        res.draw();

        res
    }

    pub fn render(&mut self, passive_indicator: u32, passive_counter: u32, time: Duration) {
        if self.passive_indicator != passive_indicator
            || self.passive_counter != passive_counter
            || self.time != time
        {
            self.passive_indicator = passive_indicator;

            let color: sdl2::pixels::Color = match passive_counter {
                0 => colors::PASSIVE_RED,
                1..11 => {
                    if time.subsec_millis() > 500 {
                        colors::PASSIVE_RED
                    } else {
                        colors::BACKGROUND
                    }
                }
                11.. => colors::PASSIVE_YELLOW,
            };

            self.passive_indicator_widget.set_x(
                self.position.x + self.position.width as i32 / 2
                    - self.position.width as i32 * passive_indicator as i32 / 2000,
            );
            self.passive_indicator_widget
                .set_width(self.position.width * passive_indicator / 1000);

            self.passive_indicator_widget.render(color);
        }
    }

    pub fn draw(&mut self) {
        self.passive_indicator_widget.draw();
    }
}
