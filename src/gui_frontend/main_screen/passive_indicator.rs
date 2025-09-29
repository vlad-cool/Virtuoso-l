use std::time::Duration;

use sdl2;
use sdl2::pixels::Color;

use crate::match_info::{MatchInfo, TimerController};
use crate::gui_frontend::colors;
use crate::gui_frontend::layout_structure::RectangleProperties;
use crate::gui_frontend::widgets::Indicator;
use crate::gui_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    passive_indicator_widget: Indicator<'a>,
    position: RectangleProperties,

    timer: TimerController,
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
            timer: TimerController::new(),
            updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.timer != data.timer_controller {
            self.timer = data.timer_controller;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        {
            let passive_timer: Duration = self.timer.get_passive_timer();
            let (color, passive_indicator): (Color, i32) = match passive_timer.as_secs() {
                0 => (colors::PASSIVE_RED, 1000),
                1..10 => (
                    if self.timer.get_main_time().subsec_millis() > 500
                        || !self.timer.is_timer_running()
                    {
                        colors::PASSIVE_RED
                    } else {
                        colors::BACKGROUND
                    },
                    1000,
                ),
                10.. => (
                    colors::PASSIVE_YELLOW,
                    1000 - ((passive_timer - Duration::from_secs(10)).as_millis() / 50) as i32,
                ),
            };

            self.passive_indicator_widget.set_x(
                self.position.x + self.position.width as i32 / 2
                    - self.position.width as i32 * passive_indicator / 2000,
            );
            self.passive_indicator_widget
                .set_width(self.position.width * passive_indicator as u32 / 1000);

            self.passive_indicator_widget.render(color);
            self.updated = false;
        }
        self.passive_indicator_widget.draw();
    }
}
