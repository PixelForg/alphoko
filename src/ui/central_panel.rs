use std::fs;

use eframe::egui::{self, CentralPanel, Image, Modal, TextEdit, Ui, Vec2};

use crate::db::{MangaPanels, add_manga_panel_to_db, retrieve_manga_panels_from_db};
use crate::{app::MyApp, ui::constants::MANGA_PANELS_SAVE_FOLDER};

impl MyApp {
    fn clear_image_state(&mut self, ctx: &egui::Context, image_uri: &String) {
        self.added_image_file_path.clear();
        self.added_image_text.clear();
        ctx.forget_image(image_uri);
    }

    fn draw_add_manga_panel_modal(&mut self, ctx: &egui::Context) {
        if !self.added_image_file_path.is_empty() {
            Modal::new("modal".into()).show(ctx, |ui| {
                ui.set_width(500.0);
                ui.vertical_centered(|ui| ui.heading("Add manga panel"));
                let uri = format!("file://{}", &self.added_image_file_path);
                let image = Image::new(&uri).max_size(Vec2 { x: 200.0, y: 200.0 });
                ui.add(image);
                ui.label("Enter manga panel text");
                ui.add(TextEdit::multiline(&mut self.added_image_text).desired_width(200.0));
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(!self.added_image_text.is_empty(), |ui| {
                        let file_name = self.added_image_file_path.split("/").into_iter().last();
                        if ui.button("Save image").clicked() {
                            let manga_panel_file_path = format!(
                                "{}/{}",
                                MANGA_PANELS_SAVE_FOLDER,
                                file_name.unwrap_or("image.jpg")
                            );
                            let _ =
                                match fs::copy(&self.added_image_file_path, &manga_panel_file_path)
                                {
                                    Ok(_) => add_manga_panel_to_db(
                                        &self.database_connection,
                                        &manga_panel_file_path,
                                        &self.added_image_text,
                                        &"Yokohama Kaidashii Kikou".to_owned(), // TODO: Need to add combobox with searhc for manga names
                                    ),
                                    Err(_) => panic!("Failed to copy file"),
                                };
                            self.clear_image_state(ctx, &uri);
                        }
                    });
                    if ui.button("Close without saving").clicked() {
                        self.clear_image_state(ctx, &uri);
                    }
                });
            });
        }
    }

    fn draw_manga_panels_gallery(&mut self, ui: &mut Ui) {
        let manga_panels = retrieve_manga_panels_from_db(&self.database_connection);
        match manga_panels {
            Ok(manga_panels_vec) => {
                ui.horizontal_wrapped(|ui| {
                    for manga_panel in manga_panels_vec {
                        let MangaPanels {
                            manga_panel_file_path,
                            manga_panel_text,
                            number_of_times_copied,
                            manga_name,
                        } = manga_panel;
                        let uri = format!("file://{}", &manga_panel_file_path);
                        let image = Image::new(&uri)
                            // Not sure why but I need to add this otherwise the images are very small for some reason, like icons
                            .fit_to_original_size(1.0)
                            .max_height(200.0);
                        ui.add(image);
                        println!(
                            "{},{},{},{}",
                            manga_panel_file_path,
                            manga_panel_text,
                            number_of_times_copied,
                            manga_name
                        );
                    }
                });
            }
            Err(err) => println!("{}", err),
        }
    }

    pub fn draw_central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.draw_add_manga_panel_modal(ctx);
            ui.label(&self.keywords_search_text);
            ui.label(&self.manga_name_search_text);
            /*
            Initially I had put this under a condition and I was negating the
            condition when I was done showing the images, however that just made
            the images disappear super fast, since this is immediate mode UI
            */
            self.draw_manga_panels_gallery(ui)
        });
    }
}
