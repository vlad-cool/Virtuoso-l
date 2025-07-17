use crate::match_info;
use eframe::{
    egui,
    epaint::text::{FontId, FontInsert, FontTweak, InsertFontFamily},
};
use egui_alignments::center_vertical;
use std::sync::MutexGuard;

fn draw_left_score(ui: &mut egui::Ui, layout: &crate::layout_structure::Layout, score: u32) {
    let rect = egui::Rect::from_min_max(
        egui::Pos2 { x: 0.0, y: 0.0 },
        egui::Pos2 {
            x: 1000.0,
            y: 1000.0,
        },
    );
    // ui.allocate_ui_at_rect(layout.score_l_l.to_rect(), |ui| {

    // egui::Label::new("sdf").

    // // ui.allocate_ui_at_rect(rect, |ui| {
    // //     ui.label(
    // //         egui::RichText::new(if score < 10 {
    // //             format!("{}", score)
    // //         } else {
    // //             format!("{}", score / 10)
    // //         })
    // //         .color(egui::Color32::BLUE)
    // //         .font(FontId::proportional(layout.score_l_l.font_size))
    // //         .background_color(egui::Color32::LIGHT_RED),
    // //     )
    // // });

    ui.allocate_ui_at_rect(rect, |ui| {
        ui.label(
            egui::RichText::new(if score < 10 {
                format!("{}", score)
            } else {
                format!("{}", score / 10)
            })
            .color(egui::Color32::BLUE)
            .font(FontId::proportional(layout.score_l_l.font_size))
            .background_color(egui::Color32::LIGHT_RED),
        )
    });
    ui.allocate_ui_at_rect(layout.score_l_r.to_rect(), |ui| {
        ui.label(
            egui::RichText::new(if score < 10 {
                format!("")
            } else {
                format!("{}", score % 10)
            })
            .color(egui::Color32::BLUE)
            .font(FontId::proportional(layout.score_l_r.font_size)),
        )
    });

    let rect: egui::Rect = layout.score_l_r.to_rect();

    ui.painter().rect(
        rect,
        0,                                                 // No rounded corners
        egui::Color32::LIGHT_GREEN,                        // Fill color
        egui::Stroke::new(2.0, egui::Color32::DARK_GREEN), // Border
        egui::StrokeKind::Inside,
    );
}

fn draw_right_score(ui: &mut egui::Ui, layout: &crate::layout_structure::Layout, score: u32) {
    ui.allocate_ui_at_rect(layout.score_r_l.to_rect(), |ui| {
        ui.label(
            egui::RichText::new(if score < 10 {
                format!("")
            } else {
                format!("{}", score / 10)
            })
            .color(egui::Color32::BLUE)
            .font(FontId::proportional(layout.score_r_l.font_size)),
        )
    });
    ui.allocate_ui_at_rect(layout.score_r_r.to_rect(), |ui| {
        ui.label(
            egui::RichText::new(format!("{}", score % 10))
                .color(egui::Color32::BLUE)
                .font(FontId::proportional(layout.score_r_r.font_size)),
        )
    });
}

pub fn draw_score(
    ui: &mut egui::Ui,
    layout: &crate::layout_structure::Layout,
    data: &MutexGuard<'_, match_info::MatchInfo>,
) {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        draw_left_score(ui, layout, data.left_fencer.score);
        draw_right_score(ui, layout, data.right_fencer.score);
    });
}
