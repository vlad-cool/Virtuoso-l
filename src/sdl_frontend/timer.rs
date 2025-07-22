use sdl2;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::colors;
use crate::match_info::Priority;
use crate::sdl_frontend::widgets::Label;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

pub struct Drawer<'a> {
    timer_0_widget: Label<'a>,
    timer_1_widget: Label<'a>,
    timer_2_widget: Label<'a>,
    timer_3_widget: Label<'a>,

    time: Duration,
    timer_running: bool,
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
            .load_font_from_rwops(rwops, layout.timer_m.font_size as u16)
            .unwrap_with_logger(logger);
        let font: Rc<sdl2::ttf::Font<'a, 'a>> = Rc::new(font);

        let mut res: Drawer<'a> = Self {
            timer_0_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.timer_m,
                logger,
            ),
            timer_1_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.timer_colon,
                logger,
            ),
            timer_2_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.timer_d,
                logger,
            ),
            timer_3_widget: Label::new(
                canvas.clone(),
                texture_creator,
                font.clone(),
                layout.timer_s,
                logger,
            ),
            time: Duration::from_secs(0),
            timer_running: true,
        };

        res.render(Duration::from_secs(60 * 3), false, Priority::None);
        res.draw();

        res
    }

    pub fn render(&mut self, time: Duration, timer_running: bool, priority: Priority) {
        if self.time != time || self.timer_running != timer_running {
            self.time = time;
            self.timer_running = timer_running;

            let colon: String = if !timer_running || time.subsec_millis() > 500 {
                ":".to_string()
            } else {
                " ".to_string()
            };

            let time_str: String = if time.as_secs() >= 10 {
                let minutes: u64 = time.as_secs() / 60;
                let seconds: u64 = time.as_secs() % 60;

                format!("{}{}{}{}", minutes, colon, seconds / 10, seconds % 10)
            } else {
                let seconds: u64 = time.as_secs();
                let centiseconds: u32 = time.subsec_millis() / 10;

                format!(
                    "{}{}{}{}",
                    seconds,
                    colon,
                    centiseconds / 10,
                    centiseconds % 10
                )
            };

            let color: sdl2::pixels::Color = if timer_running {
                if time.as_secs() > 10 {
                    colors::TIMER_WHITE
                } else {
                    colors::TIMER_BLUE
                }
            } else {
                colors::TIMER_ORANGE
            };

            let colon_color: sdl2::pixels::Color = if timer_running {
                match priority {
                    Priority::Left => colors::PRIORITY_RED,
                    Priority::None => color,
                    Priority::Right => colors::PRIORITY_GREEN,
                }
            } else {
                color
            };

            self.timer_0_widget.render(&time_str[0..1], color);
            self.timer_1_widget.render(&time_str[1..2], colon_color);
            self.timer_2_widget.render(&time_str[2..3], color);
            self.timer_3_widget.render(&time_str[3..4], color);
        }
    }

    pub fn draw(&mut self) {
        self.timer_0_widget.draw();
        self.timer_1_widget.draw();
        self.timer_2_widget.draw();
        self.timer_3_widget.draw();
    }
}
