use sdl2;
use sdl2::pixels::Color;
use std::time::Duration;

use crate::sdl_frontend::colors;
use crate::sdl_frontend::layout_structure::RectangleProperties;
use crate::match_info::MatchInfo;
use crate::sdl_frontend::widgets::Indicator;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    passive_indicator_widget: Indicator<'a>,
    position: RectangleProperties,

    passive_indicator: u32,
    passive_counter: u32,
    time: Duration,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        Self {
            passive_indicator_widget: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.passive_indicator,
                context.logger,
            ),
            position: context.layout.passive_indicator.clone(),
            passive_indicator: 0,
            passive_counter: 60,
            time: Duration::from_secs(0),
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.passive_indicator != data.passive_timer.get_indicator()
            || self.passive_counter != data.passive_timer.get_counter()
            || self.time != data.timer_controller.get_time()
        {
            self.passive_indicator = data.passive_timer.get_indicator();
            self.passive_counter = data.passive_timer.get_counter();
            self.time = data.timer_controller.get_time();
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated {
            let color: Color = match self.passive_counter {
                0 => colors::PASSIVE_RED,
                1..11 => {
                    if self.time.subsec_millis() > 500 {
                        colors::PASSIVE_RED
                    } else {
                        colors::BACKGROUND
                    }
                }
                11.. => colors::PASSIVE_YELLOW,
            };

            self.passive_indicator_widget.set_x(
                self.position.x + self.position.width as i32 / 2
                    - self.position.width as i32 * self.passive_indicator as i32 / 2000,
            );
            self.passive_indicator_widget
                .set_width(self.position.width * self.passive_indicator / 1000);

            self.passive_indicator_widget.render(color);
            self.updated = false;
        }
        self.passive_indicator_widget.draw();
    }
}
