use crate::match_info::{MatchInfo, TimerController};
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::Indicator;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    l_color_widget_active: Indicator<'a>,
    l_white_widget_active: Indicator<'a>,
    r_color_widget_active: Indicator<'a>,
    r_white_widget_active: Indicator<'a>,
    l_color_widget_passive: Indicator<'a>,
    l_white_widget_passive: Indicator<'a>,
    r_color_widget_passive: Indicator<'a>,
    r_white_widget_passive: Indicator<'a>,

    l_color_led_on: bool,
    l_white_led_on: bool,
    r_color_led_on: bool,
    r_white_led_on: bool,
    timer: Option<TimerController>,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let mut res: Drawer<'a> = Self {
            l_color_widget_active: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.left_color_indicator,
                context.logger,
            ),
            l_white_widget_active: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.left_white_indicator,
                context.logger,
            ),
            r_color_widget_active: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.right_color_indicator,
                context.logger,
            ),
            r_white_widget_active: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.right_white_indicator,
                context.logger,
            ),
            l_color_widget_passive: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.left_color_indicator,
                context.logger,
            ),
            l_white_widget_passive: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.left_white_indicator,
                context.logger,
            ),
            r_color_widget_passive: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.right_color_indicator,
                context.logger,
            ),
            r_white_widget_passive: Indicator::new(
                context.canvas.clone(),
                context.texture_creator,
                context.layout.right_white_indicator,
                context.logger,
            ),

            l_color_led_on: false,
            l_white_led_on: false,
            r_color_led_on: false,
            r_white_led_on: false,
            timer: None,
        };

        res.l_color_widget_active.render(colors::COLOR_LABELS_RED);
        res.l_white_widget_active.render(colors::WHITE_LABELS_LIGHT);
        res.r_color_widget_active.render(colors::COLOR_LABELS_GREEN);
        res.r_white_widget_active.render(colors::WHITE_LABELS_LIGHT);
        res.l_color_widget_passive
            .render(colors::COLOR_LABELS_DARK_RED);
        res.l_white_widget_passive.render(colors::WHITE_LABELS_DARK);
        res.r_color_widget_passive
            .render(colors::COLOR_LABELS_DARK_GREEN);
        res.r_white_widget_passive.render(colors::WHITE_LABELS_DARK);

        return res;
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        self.timer = Some(data.timer_controller);

        self.l_color_led_on = data.left_fencer.color_light;
        self.l_white_led_on = data.left_fencer.white_light;
        self.r_color_led_on = data.right_fencer.color_light;
        self.r_white_led_on = data.right_fencer.white_light;
    }

    fn render(&mut self) {
        let left_medical: bool = if let Some(timer) = self.timer {
            timer.medical_left_flash()
        } else {
            false
        };
        let right_medical = if let Some(timer) = self.timer {
            timer.medical_right_flash()
        } else {
            false
        };

        if self.l_color_led_on || left_medical {
            self.l_color_widget_active.draw();
        } else {
            self.l_color_widget_passive.draw();
        }
        if self.l_white_led_on {
            self.l_white_widget_active.draw();
        } else {
            self.l_white_widget_passive.draw();
        }

        if self.r_color_led_on || right_medical {
            self.r_color_widget_active.draw();
        } else {
            self.r_color_widget_passive.draw();
        }
        if self.r_white_led_on {
            self.r_white_widget_active.draw();
        } else {
            self.r_white_widget_passive.draw();
        }
    }
}
