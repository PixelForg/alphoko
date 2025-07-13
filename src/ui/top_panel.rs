use eframe::egui::{
    self, Align, Button, Color32, CornerRadius, Frame, Layout, RichText, Stroke, TextEdit,
    TopBottomPanel, Ui, vec2,
};
use rfd::FileDialog;

use crate::{
    app::{MyApp, SearchBarText},
    ui::constants::TOP_PANEL_ELEMENTS_HEIGHT,
};

impl MyApp {
    fn draw_default_frame(&mut self) -> Frame {
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
    fn draw_search_bar(&mut self, ui: &mut Ui, width: f32, search_bar_option: SearchBarText) {
        self.draw_default_frame()
            .fill(Color32::from_rgb(23, 23, 23))
            .show(ui, |ui| {
                let search_text_to_edit = match search_bar_option {
                    SearchBarText::Keywords(default_text) => {
                        (&mut self.keywords_search_text, default_text)
                    }
                    SearchBarText::MangaName(default_text) => {
                        (&mut self.manga_name_search_text, default_text)
                    }
                };
                ui.set_height(TOP_PANEL_ELEMENTS_HEIGHT);
                ui.add(
                    TextEdit::singleline(search_text_to_edit.0)
                        .desired_width(width)
                        .hint_text(search_text_to_edit.1)
                        .frame(false),
                );
            });
    }

    fn draw_add_image_button(&mut self, ui: &mut Ui, width: f32) {
        self.draw_default_frame()
            .fill(Color32::WHITE)
            .show(ui, |ui| {
                ui.set_width(width);
                // This is necessary to center text in button
                ui.vertical_centered(|ui| {
                    if ui
                        .add(
                            Button::new(
                                RichText::new("Upload manga panel")
                                    .size(13.0)
                                    .color(Color32::BLACK),
                            )
                            .min_size(vec2(120.0, TOP_PANEL_ELEMENTS_HEIGHT))
                            .frame(false),
                        )
                        .clicked()
                    {
                        if let Some(added_image_file_path) = FileDialog::new().pick_file() {
                            self.added_image_file_path =
                                added_image_file_path.display().to_string();
                        }
                    }
                })
            });
    }

    pub fn draw_top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let window_width = ui.available_rect_before_wrap().width();
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        self.draw_search_bar(
                            ui,
                            window_width * 0.50,
                            SearchBarText::Keywords("Enter search text".to_owned()),
                        );
                        self.draw_search_bar(
                            ui,
                            window_width * 0.30,
                            SearchBarText::MangaName("Enter manga name".to_owned()),
                        );
                        self.draw_add_image_button(ui, window_width * 0.10);
                    })
                });
                ui.label(&self.added_image_file_path);
            });
    }
}
