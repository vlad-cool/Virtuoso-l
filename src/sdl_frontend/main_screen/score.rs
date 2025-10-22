use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors;
use crate::sdl_frontend::widgets::{Label, LabelHashKey, LabelTextureCache};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    score_l_l_widget: Label<'a>,
    score_l_r_widget: Label<'a>,
    score_r_l_widget: Label<'a>,
    score_r_r_widget: Label<'a>,
    texture_cache: LabelTextureCache<'a>,

    score_l: u32,
    score_l_updated: bool,
    score_l_update_time: Option<Instant>,
    score_r: u32,
    score_r_updated: bool,
    score_r_update_time: Option<Instant>,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let font: Rc<Font<'_, '_>> = context.get_font(context.layout.score_l_l.font_size);
        let mut texture_cache: LabelTextureCache<'a> = LabelTextureCache::new();

        for char in "0123456789 ".chars() {
            for color in [colors::SCORE_LEFT, colors::SCORE_RIGHT] {
                let key: LabelHashKey = LabelHashKey {
                    color,
                    text: char.to_string(),
                };
                texture_cache.get(key, context.texture_creator, font.clone(), context.logger);
            }
        }

        Self {
            score_l_l_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.score_l_l,
                context.logger,
            ),
            score_l_r_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.score_l_r,
                context.logger,
            ),
            score_r_l_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.score_r_l,
                context.logger,
            ),
            score_r_r_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                context.layout.score_r_r,
                context.logger,
            ),
            texture_cache,

            score_l: 0,
            score_l_updated: true,
            score_l_update_time: None,
            score_r: 0,
            score_r_updated: true,
            score_r_update_time: None,
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        if self.score_l != data.left_fencer.score {
            self.score_l = data.left_fencer.score;
            self.score_l_updated = true;
            self.score_l_update_time = data.left_fencer.score_auto_updated;
        }
        if self.score_r != data.right_fencer.score {
            self.score_r = data.right_fencer.score;
            self.score_r_updated = true;
            self.score_r_update_time = data.right_fencer.score_auto_updated;
        }
    }

    fn render(&mut self) {
        if self.score_l_updated {
            let score_l_l_text: String = if self.score_l < 10 {
                format!("{}", self.score_l)
            } else {
                format!("{}", self.score_l / 10)
            };
            self.score_l_l_widget.render(
                score_l_l_text,
                colors::SCORE_LEFT,
                Some(&mut self.texture_cache),
            );

            let score_l_r_text: String = if self.score_l < 10 {
                " ".to_string()
            } else {
                format!("{}", self.score_l % 10)
            };
            self.score_l_r_widget.render(
                score_l_r_text,
                colors::SCORE_LEFT,
                Some(&mut self.texture_cache),
            );
            self.score_l_updated = false;
        }

        if self.score_r_updated {
            let score_r_l_text: String = if self.score_r < 10 {
                " ".to_string()
            } else {
                format!("{}", self.score_r / 10)
            };
            self.score_r_l_widget.render(
                score_r_l_text,
                colors::SCORE_RIGHT,
                Some(&mut self.texture_cache),
            );

            let score_r_r_text: String = if self.score_r < 10 {
                format!("{}", self.score_r)
            } else {
                format!("{}", self.score_r % 10)
            };
            self.score_r_r_widget.render(
                score_r_r_text,
                colors::SCORE_RIGHT,
                Some(&mut self.texture_cache),
            );
            self.score_r_updated = false;
        }

        let draw_l: bool = if let Some(time) = self.score_l_update_time {
            let time: Duration = time.elapsed();
            if time > Duration::from_millis(500) && time < Duration::from_millis(1000) {
                true
            } else if time > Duration::from_millis(1500) {
                true
            } else {
                false
            }
        } else {
            true
        };
        let draw_r: bool = if let Some(time) = self.score_r_update_time {
            let time: Duration = time.elapsed();
            if time > Duration::from_millis(500) && time < Duration::from_millis(1000) {
                true
            } else if time > Duration::from_millis(1500) {
                true
            } else {
                false
            }
        } else {
            true
        };
        if draw_l {
            self.score_l_l_widget.draw();
            self.score_l_r_widget.draw();
        }
        if draw_r {
            self.score_r_l_widget.draw();
            self.score_r_r_widget.draw();
        }
    }
}
