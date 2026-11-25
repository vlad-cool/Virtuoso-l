use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use rust_iso3166::{self, CountryCode};
use sdl2::ttf::Font;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::INACTIVE;
use crate::sdl_frontend::widgets::{Image, Label};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_photo_widget: Image<'a>,
    right_photo_widget: Image<'a>,
    left_photo_empty: Label<'a>,
    right_photo_empty: Label<'a>,
    left_photo: Option<u32>,
    left_photo_present: bool,
    left_photo_updated: bool,
    right_photo: Option<u32>,
    right_photo_present: bool,
    right_photo_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.left_nation.font_size);

            let mut left_photo_empty: Label<'_> = Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                layout.left_photo.to_text_properties(0),
                context.logger,
            );
            let mut right_photo_empty: Label<'_> = Label::new(
                context.canvas.clone(),
                context.texture_creator,
                font.clone(),
                layout.right_photo.to_text_properties(0),
                context.logger,
            );

            left_photo_empty.render("Fencer's\nphoto".to_string(), INACTIVE, None);
            right_photo_empty.render("Fencer's\nphoto".to_string(), INACTIVE, None);

            Some(Self {
                left_photo_widget: Image::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    layout.left_flag,
                    context.logger,
                ),
                right_photo_widget: Image::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    layout.right_flag,
                    context.logger,
                ),
                left_photo_empty,
                right_photo_empty,

                left_photo: None,
                left_photo_present: false,
                left_photo_updated: true,
                right_photo: None,
                right_photo_present: false,
                right_photo_updated: true,
            })
        } else {
            None
        }
    }

    fn get_flag_path(code: Option<CountryCode>) -> Option<PathBuf> {
        let code: CountryCode = if let Some(code) = code {
            code
        } else {
            return None;
        };

        if cfg!(feature = "embeded_device") {
            let path_1: PathBuf = PathBuf::from_str(
                format!("~/Virtuoso/flags/{}.png", code.alpha2.to_lowercase()).as_str(),
            )
            .unwrap();
            let path_2: PathBuf =
                PathBuf::from_str(format!("~/flags/{}.png", code.alpha2.to_lowercase()).as_str())
                    .unwrap();

            if path_1.exists() {
                Some(path_1)
            } else if path_2.exists() {
                Some(path_2)
            } else {
                None
            }
        } else {
            let path: PathBuf =
                PathBuf::from_str(format!("flags/{}.png", code.alpha2.to_lowercase()).as_str())
                    .unwrap();

            if path.exists() { Some(path) } else { None }
        }
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {}

    fn render(&mut self) {
        self.left_photo_empty.draw();
        self.right_photo_empty.draw();
    }
}
