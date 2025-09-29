use sdl2;

use crate::match_info::MatchInfo;
use crate::gui_frontend::colors;
use crate::gui_frontend::widgets::Indicator;
use crate::gui_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    l_color_widget: Indicator<'a>,
    l_white_widget: Indicator<'a>,
    r_color_widget: Indicator<'a>,
    r_white_widget: Indicator<'a>,

    l_color_led_on: bool,
    l_color_led_updated: bool,
    l_white_led_on: bool,
    l_white_led_updated: bool,
    r_color_led_on: bool,
    r_color_led_updated: bool,
    r_white_led_on: bool,
    r_white_led_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        Self {
            l_color_widget: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.left_color_indicator,
                context.logger,
            ),
            l_white_widget: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.left_white_indicator,
                context.logger,
            ),
            r_color_widget: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.right_color_indicator,
                context.logger,
            ),
            r_white_widget: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.right_white_indicator,
                context.logger,
            ),

            l_color_led_on: false,
            l_color_led_updated: true,
            l_white_led_on: false,
            l_white_led_updated: true,
            r_color_led_on: false,
            r_color_led_updated: true,
            r_white_led_on: false,
            r_white_led_updated: true,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.l_color_led_on != data.left_fencer.color_light {
            self.l_color_led_on = data.left_fencer.color_light;
            self.l_color_led_updated = true;
        }
        if self.l_white_led_on != data.left_fencer.white_light {
            self.l_white_led_on = data.left_fencer.white_light;
            self.l_white_led_updated = true;
        }
        if self.r_color_led_on != data.right_fencer.color_light {
            self.r_color_led_on = data.right_fencer.color_light;
            self.r_color_led_updated = true;
        }
        if self.r_white_led_on != data.right_fencer.white_light {
            self.r_white_led_on = data.right_fencer.white_light;
            self.r_white_led_updated = true;
        }
    }

    fn render(&mut self) {
        if self.l_color_led_updated {
            let color: sdl2::pixels::Color = if self.l_color_led_on {
                colors::COLOR_LABELS_RED
            } else {
                colors::COLOR_LABELS_DARK_RED
            };
            self.l_color_widget.render(color);
            self.l_color_led_updated = false;
        }
        if self.l_white_led_updated {
            let color: sdl2::pixels::Color = if self.l_white_led_on {
                colors::WHITE_LABELS_LIGHT
            } else {
                colors::WHITE_LABELS_DARK
            };
            self.l_white_widget.render(color);
            self.l_white_led_updated = false;
        }
        if self.r_color_led_updated {
            let color: sdl2::pixels::Color = if self.r_color_led_on {
                colors::COLOR_LABELS_GREEN
            } else {
                colors::COLOR_LABELS_DARK_GREEN
            };
            self.r_color_widget.render(color);
            self.r_color_led_updated = false;
        }
        if self.r_white_led_updated {
            let color: sdl2::pixels::Color = if self.r_white_led_on {
                colors::WHITE_LABELS_LIGHT
            } else {
                colors::WHITE_LABELS_DARK
            };
            self.r_white_widget.render(color);
            self.r_white_led_updated = false;
        }

        self.l_color_widget.draw();
        self.l_white_widget.draw();
        self.r_color_widget.draw();
        self.r_white_widget.draw();
    }
}
