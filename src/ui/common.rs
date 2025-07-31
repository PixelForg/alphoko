use eframe::egui::{Color32, CornerRadius, Frame, Stroke, TextEdit, Ui};

use crate::ui::constants::TOP_PANEL_ELEMENTS_HEIGHT;

pub fn draw_default_frame() -> Frame {
    Frame::new()
        .stroke(Stroke {
            width: 2.0,
            color: Color32::from_rgb(37, 37, 38),
        })
        .corner_radius(CornerRadius {
            nw: 10,
            ne: 10,
            sw: 10,
            se: 10,
        })
}

pub fn draw_search_bar(
    ui: &mut Ui,
    width: f32,
    search_text_to_edit: &mut String,
    default_text: &String,
) {
    draw_default_frame()
        .fill(Color32::from_rgb(23, 23, 23))
        .show(ui, |ui| {
            ui.set_height(TOP_PANEL_ELEMENTS_HEIGHT);
            ui.add(
                TextEdit::singleline(search_text_to_edit)
                    .desired_width(width)
                    .hint_text(default_text)
                    .frame(false),
            );
        });
}
