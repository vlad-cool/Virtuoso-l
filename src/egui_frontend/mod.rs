use eframe::{
    egui,
    epaint::text::{FontId, FontInsert, InsertFontFamily},
};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use crate::match_info;
use crate::modules;
use crate::virtuoso_logger::Logger;
use crate::{
    hw_config::{HardwareConfig, Resolution},
    modules::VirtuosoModule,
};
use crate::{layout_structure, layouts};

mod score;

impl crate::layout_structure::TextProperties {
    fn to_rect(&self) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.x, self.y),
            egui::vec2(self.width, self.height),
        )
    }
}

impl crate::layout_structure::RectangleProperties {
    fn to_rect(&self) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.x, self.y),
            egui::vec2(self.width, self.height),
        )
    }
}

const MESSAGE_DISPLAY_TIME: Duration = Duration::from_secs(2);

pub struct EguiFrontend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    hw_config: HardwareConfig,
    logger: Logger,
    layout: layout_structure::Layout,
}

impl EguiFrontend {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        hw_config: HardwareConfig,
        logger: Logger,
    ) -> Self {
        let layout = match hw_config.display.resolution {
            Resolution::Res1920X1080 => layouts::LAYOUT_1920X1080,
            Resolution::Res1920X550 => layouts::LAYOUT_1920X550,
            Resolution::Res1920X480 => layouts::LAYOUT_1920X480,
            Resolution::Res1920X360 => layouts::LAYOUT_1920X360,
        };

        Self {
            match_info,
            hw_config,
            logger,
            layout,
        }
    }
}

impl VirtuosoModule for EguiFrontend {
    fn run(mut self) {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                // .with_inner_size([self.layout.background.width, self.layout.background.height])
                .with_inner_size([self.layout.background.width, 1080.0])
                .with_resizable(false),
            ..Default::default()
        };

        eframe::run_native(
            "Virtuoso",
            options,
            Box::new(|cc| {
                cc.egui_ctx.add_font(FontInsert::new(
                    "AgencyB",
                    egui::FontData::from_static(include_bytes!("../../assets/AGENCYB.ttf")),
                    vec![
                        InsertFontFamily {
                            family: egui::FontFamily::Proportional,
                            priority: egui::epaint::text::FontPriority::Highest,
                        },
                        InsertFontFamily {
                            family: egui::FontFamily::Monospace,
                            priority: egui::epaint::text::FontPriority::Lowest,
                        },
                    ],
                ));

                Ok(Box::new(self))
            }),
        );
    }
}

impl eframe::App for EguiFrontend {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let data: MutexGuard<'_, match_info::MatchInfo> = self.match_info.lock().unwrap();

            score::draw_score(ui, &self.layout, &data);
        });
    }
}
