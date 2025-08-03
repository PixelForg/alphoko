use eframe::egui::{
    Button, Color32, Context, Popup, PopupCloseBehavior, Response, RichText, ScrollArea, SidePanel,
    Ui, vec2,
};
use rfd::FileDialog;

use crate::{
    app::MyApp,
    ui::{
        common::{draw_default_frame, draw_search_bar, get_fuzzy_search_options},
        constants::{AUTOCOMPLETE_POPUP_MAX_HEIGHT, SIDE_PANEL_ELEMENTS_HEIGHT},
    },
};

impl MyApp {
    fn draw_add_image_button(&mut self, ui: &mut Ui) {
        draw_default_frame().fill(Color32::WHITE).show(ui, |ui| {
            // This is necessary to center text in button
            ui.vertical_centered(|ui| {
                if ui
                    .add(
                        Button::new(
                            RichText::new("Upload manga panel")
                                .size(13.0)
                                .color(Color32::BLACK),
                        )
                        .min_size(vec2(120.0, SIDE_PANEL_ELEMENTS_HEIGHT))
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

    pub fn draw_side_panel(&mut self, ctx: &Context) {
        SidePanel::left("search-panel")
            .exact_width(300.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let window_width = ui.available_width();
                    let height = ui.available_height();
                    println!("{}", height);
                    // TODO : Try to get the popups to have same width as search bar
                    let manga_keywords_search_bar_width = window_width * 0.25;
                    let manga_name_search_bar_width = window_width * 0.25;
                    let manga_keywords_search_bar_response = draw_search_bar(
                        ui,
                        &mut self.keywords_search_text,
                        &"Enter Keywords".to_owned(),
                        true,
                    );
                    self.draw_keywords_search_popup(
                        manga_keywords_search_bar_response,
                        manga_keywords_search_bar_width,
                    );
                    ui.add_space((height - 140.0) / 2.0);
                    let manga_name_search_bar_response = draw_search_bar(
                        ui,
                        &mut self.manga_name_search_text,
                        &"Enter Keywords".to_owned(),
                        true,
                    );
                    self.draw_manga_names_search_popup(
                        manga_name_search_bar_response,
                        manga_name_search_bar_width,
                    );
                    ui.add_space((height - 140.0) / 2.0);
                    self.draw_add_image_button(ui);
                });
            });
    }
}
