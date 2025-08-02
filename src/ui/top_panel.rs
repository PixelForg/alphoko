use eframe::egui::{
    self, Align, Button, Color32, Layout, Popup, PopupCloseBehavior, Response, RichText,
    ScrollArea, TopBottomPanel, Ui, vec2,
};
use rfd::FileDialog;

use crate::{
    app::MyApp,
    ui::{
        common::{draw_default_frame, draw_search_bar, get_fuzzy_search_options},
        constants::{AUTOCOMPLETE_POPUP_MAX_HEIGHT, TOP_PANEL_ELEMENTS_HEIGHT},
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

    fn draw_keywords_search_popup(
        &mut self,
        manga_keywords_search_bar_response: Option<Response>,
        manga_keywords_search_bar_width: f32,
    ) {
        if let Some(response) = manga_keywords_search_bar_response {
            if self.keywords_search_text.is_empty() {
                self.manga_keyword_was_chosen = false;
            }
            let manga_keywords_list_with_score =
                get_fuzzy_search_options(&self.manga_panels_text_list, &self.keywords_search_text);
            Popup::menu(&response)
                .open(
                    !self.keywords_search_text.is_empty()
                        && !manga_keywords_list_with_score.is_empty()
                        && !self.manga_keyword_was_chosen,
                )
                .close_behavior(PopupCloseBehavior::IgnoreClicks)
                .show(|ui| {
                    ui.set_min_width(manga_keywords_search_bar_width);
                    ui.set_max_height(AUTOCOMPLETE_POPUP_MAX_HEIGHT);
                    ScrollArea::vertical().show(ui, |ui| {
                        for manga_panel_keywords in manga_keywords_list_with_score {
                            if ui.button(&manga_panel_keywords.1).clicked() {
                                self.keywords_search_text = manga_panel_keywords.1;
                                self.manga_keyword_was_chosen = true;
                            }
                        }
                    })
                });
        }
    }

    fn draw_manga_names_search_popup(
        &mut self,
        manga_name_search_bar_response: Option<Response>,
        manga_name_search_bar_width: f32,
    ) {
        if let Some(response) = manga_name_search_bar_response {
            if self.manga_name_search_text.is_empty() {
                self.manga_name_was_chosen = false;
            }
            let manga_names_with_score =
                get_fuzzy_search_options(&self.manga_names_list, &self.manga_name_search_text);
            Popup::menu(&response)
                .open(
                    !self.manga_name_search_text.is_empty()
                        && !manga_names_with_score.is_empty()
                        && !self.manga_name_was_chosen,
                )
                .close_behavior(PopupCloseBehavior::IgnoreClicks)
                .show(|ui| {
                    ui.set_min_width(manga_name_search_bar_width);
                    ui.set_max_height(AUTOCOMPLETE_POPUP_MAX_HEIGHT);
                    ScrollArea::vertical().show(ui, |ui| {
                        for manga_name in manga_names_with_score {
                            if ui.button(&manga_name.1).clicked() {
                                self.manga_name_search_text = manga_name.1;
                                self.manga_name_was_chosen = true;
                            }
                        }
                    });
                });
        }
    }

    pub fn draw_top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let window_width = ui.available_rect_before_wrap().width();
                    let manga_keywords_search_bar_width = window_width * 0.50;
                    let manga_name_search_bar_width = window_width * 0.35;
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        let manga_keywords_search_bar_response = draw_search_bar(
                            ui,
                            manga_keywords_search_bar_width,
                            &mut self.keywords_search_text,
                            &"Enter Keywords".to_owned(),
                            true,
                        );
                        self.draw_keywords_search_popup(
                            manga_keywords_search_bar_response,
                            manga_keywords_search_bar_width,
                        );
                        let manga_name_search_bar_response = draw_search_bar(
                            ui,
                            manga_name_search_bar_width,
                            &mut self.manga_name_search_text,
                            &"Enter Keywords".to_owned(),
                            true,
                        );
                        self.draw_manga_names_search_popup(
                            manga_name_search_bar_response,
                            manga_name_search_bar_width,
                        );
                        self.draw_add_image_button(ui, window_width * 0.10);
                    })
                });
                ui.label(&self.added_image_file_path);
            });
    }
}
