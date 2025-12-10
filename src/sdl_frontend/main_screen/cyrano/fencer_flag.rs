use std::path::PathBuf;
use std::str::FromStr;

use rust_iso3166::{self, CountryCode};

use crate::match_info::MatchInfo;
use crate::sdl_frontend::widgets::Image;
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

pub struct Drawer<'a> {
    left_nation_widget: Image<'a>,
    right_nation_widget: Image<'a>,

    left_country: Option<rust_iso3166::CountryCode>,
    left_country_present: bool,
    left_country_updated: bool,
    right_country: Option<rust_iso3166::CountryCode>,
    right_country_present: bool,
    right_country_updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            Some(Self {
                left_nation_widget: Image::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    layout.left_flag,
                    context.logger,
                ),
                right_nation_widget: Image::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    layout.right_flag,
                    context.logger,
                ),

                left_country: None,
                left_country_present: false,
                left_country_updated: true,
                right_country: None,
                right_country_present: false,
                right_country_updated: true,
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
    fn update(&mut self, data: &MatchInfo) {
        let left_country: Option<rust_iso3166::CountryCode> =
            match (&data.left_fencer.nation, data.left_fencer.nation.len()) {
                (s, 2) => rust_iso3166::from_alpha2(s.as_str()),
                (s, 3) => rust_iso3166::from_alpha3(s.as_str()),
                _ => None,
            };
        let right_country: Option<rust_iso3166::CountryCode> =
            match (&data.right_fencer.nation, data.right_fencer.nation.len()) {
                (s, 2) => rust_iso3166::from_alpha2(s.as_str()),
                (s, 3) => rust_iso3166::from_alpha3(s.as_str()),
                _ => None,
            };

        if self.left_country != left_country {
            self.left_country = left_country;
            self.left_country_updated = true;
        }
        if self.right_country != right_country {
            self.right_country = right_country;
            self.right_country_updated = true;
        }
    }

    fn render(&mut self) {
        if self.left_country_updated {
            let path: Option<PathBuf> = Self::get_flag_path(self.left_country);

            self.left_country_present = path.is_some();

            if let Some(path) = path {
                self.left_nation_widget.render(path);
            }
            self.left_country_updated = false;
        }
        if self.right_country_updated {
            let path: Option<PathBuf> = Self::get_flag_path(self.right_country);

            self.right_country_present = path.is_some();

            if let Some(path) = path {
                self.right_nation_widget.render(path);
            }
            self.right_country_updated = false;
        }

        if self.left_country_present {
            self.left_nation_widget.draw();
        }
        if self.right_country_present {
            self.right_nation_widget.draw();
        }
    }
}
