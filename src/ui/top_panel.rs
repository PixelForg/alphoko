use eframe::egui::{
    self, Align, Button, Color32, Layout, Popup, PopupCloseBehavior, RichText, ScrollArea,
    TopBottomPanel, Ui, vec2,
};
use rfd::FileDialog;

use crate::{
    app::MyApp,
    ui::{
        common::{draw_default_frame, draw_search_bar, get_manga_names_options},
        constants::TOP_PANEL_ELEMENTS_HEIGHT,
    },
};

impl MyApp {
    fn draw_add_image_button(&mut self, ui: &mut Ui, width: f32) {
        draw_default_frame().fill(Color32::WHITE).show(ui, |ui| {
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
                        self.added_image_file_path = added_image_file_path.display().to_string();
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
                        draw_search_bar(
                            ui,
                            window_width * 0.50,
                            &mut self.keywords_search_text,
                            &"Enter Keywords".to_owned(),
                            false,
                        );
                        let search_bar_response = draw_search_bar(
                            ui,
                            window_width * 0.35,
                            &mut self.manga_name_search_text,
                            &"Enter Keywords".to_owned(),
                            true,
                        );
                        if let Some(response) = search_bar_response {
                            let manga_names_with_score = get_manga_names_options(
                                &self.manga_names_list,
                                &self.manga_name_search_text,
                            );
                            Popup::menu(&response)
                                .open(
                                    !self.manga_name_search_text.is_empty()
                                        && !manga_names_with_score.is_empty(),
                                )
                                .close_behavior(PopupCloseBehavior::IgnoreClicks)
                                .show(|ui| {
                                    ui.set_min_width(310.0);
                                    ScrollArea::vertical().show(ui, |ui| {
                                        for manga_name in manga_names_with_score {
                                            if ui.button(&manga_name.1).clicked() {
                                                self.manga_name_search_text = manga_name.1;
                                            }
                                        }
                                    });
                                });
                        }
                        self.draw_add_image_button(ui, window_width * 0.10);
                    })
                });
                ui.label(&self.added_image_file_path);
            });
    }
}
