use std::fs;

use eframe::egui::{
    self, Align, CentralPanel, Color32, ColorImage, Context, Image, Layout, Modal, ScrollArea,
    TextEdit, Ui, Vec2,
};
use image::{ImageError, ImageReader};

use crate::db::{
    MangaPanels, add_manga_panel_to_db, retrieve_manga_names_from_db,
    retrieve_manga_panels_from_db, retrieve_manga_panels_text_from_db,
};
use crate::ui::common::{draw_search_bar, get_fuzzy_search_options};
use crate::{app::MyApp, ui::constants::MANGA_PANELS_SAVE_FOLDER};

impl MyApp {
    fn refresh_manga_names_list_from_db(&mut self) {
        self.manga_names_list =
            retrieve_manga_names_from_db(&self.database_connection).unwrap_or(Default::default());
    }
    fn refresh_manga_panels_keywords_list_from_db(&mut self) {
        self.manga_panels_text_list = retrieve_manga_panels_text_from_db(&self.database_connection)
            .unwrap_or(Default::default())
    }

    fn clear_image_state(&mut self, ctx: &egui::Context, image_uri: &String) {
        self.added_image_file_path.clear();
        self.added_image_text.clear();
        self.add_manga_panel_modal_manga_name.clear();
        ctx.forget_image(image_uri);
    }

    fn draw_manga_names_buttons_list(&mut self, ui: &mut Ui) {
        let manga_names_with_score = get_fuzzy_search_options(
            &self.manga_names_list,
            &self.add_manga_panel_modal_manga_name,
        );
        ScrollArea::vertical().show(ui, |ui| {
            for manga_name in manga_names_with_score {
                if ui.button(&manga_name.1).clicked() {
                    self.add_manga_panel_modal_manga_name = manga_name.1;
                }
            }
        });
    }

    fn draw_add_manga_panel_modal(&mut self, ctx: &egui::Context) {
        if !self.added_image_file_path.is_empty() {
            Modal::new("modal".into()).show(ctx, |ui| {
                ui.set_width(700.0);
                ui.vertical_centered(|ui| ui.heading("Add manga panel"));
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        let uri = format!("file://{}", &self.added_image_file_path);
                        let image = Image::new(&uri)
                            .fit_to_original_size(1.0)
                            .max_size(Vec2 { x: 400.0, y: 400.0 });
                        ui.add(image);
                        ui.label("Enter manga panel text");
                        ui.add(
                            TextEdit::multiline(&mut self.added_image_text).desired_width(200.0),
                        );
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(!self.added_image_text.is_empty(), |ui| {
                                let file_name =
                                    self.added_image_file_path.split("/").into_iter().last();
                                if ui.button("Save image").clicked() {
                                    let manga_panel_file_path = format!(
                                        "{}/{}",
                                        MANGA_PANELS_SAVE_FOLDER,
                                        file_name.unwrap_or("image.jpg")
                                    );
                                    let _ = match fs::copy(
                                        &self.added_image_file_path,
                                        &manga_panel_file_path,
                                    ) {
                                        Ok(_) => add_manga_panel_to_db(
                                            &self.database_connection,
                                            &file_name.unwrap_or("image.jpg").to_owned(),
                                            &self.added_image_text.trim().to_owned(),
                                            &self.add_manga_panel_modal_manga_name,
                                        ),
                                        Err(_) => panic!("Failed to copy file"),
                                    };
                                    self.refresh_manga_names_list_from_db();
                                    self.refresh_manga_panels_keywords_list_from_db();
                                    self.clear_image_state(ctx, &uri);
                                }
                            });
                            if ui.button("Close without saving").clicked() {
                                self.clear_image_state(ctx, &uri);
                            }
                        });
                    });
                    ui.with_layout(Layout::top_down(Align::TOP), |ui| {
                        draw_search_bar(
                            ui,
                            &mut self.add_manga_panel_modal_manga_name,
                            &"Add manga name".to_owned(),
                            false,
                        );
                        self.draw_manga_names_buttons_list(ui);
                    })
                });
            });
        }
    }

    fn load_image_from_path(
        &self,
        manga_panel_file_path: &String,
    ) -> Result<ColorImage, ImageError> {
        let dynamic_image = ImageReader::open(manga_panel_file_path)?
            /*
            This is needed because even if the file is .png, it might actually be a jpg.
            Which will cause a panic here
            */
            .with_guessed_format()?
            .decode()?;
        let size = [
            dynamic_image.width() as usize,
            dynamic_image.height() as usize,
        ];
        let image_buffer = dynamic_image.to_rgba8();
        let pixels: Vec<Color32> = image_buffer
            .pixels()
            .map(|p| Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        let color_image = ColorImage {
            size,
            pixels,
            source_size: Default::default(),
        };
        Ok(color_image)
    }

    fn draw_manga_panels_gallery(&mut self, ui: &mut Ui, ctx: &Context) {
        if !self.keywords_search_text.is_empty() {
            let manga_panels = retrieve_manga_panels_from_db(
                &self.database_connection,
                &self.keywords_search_text,
                &self.manga_name_search_text,
            );
            match manga_panels {
                Ok(manga_panels_vec) => {
                    // TODO : Fix scrollbar
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for manga_panel in &manga_panels_vec {
                                let MangaPanels { file_name, .. } = manga_panel;
                                let manga_panel_file_path =
                                    format!("{}/{}", MANGA_PANELS_SAVE_FOLDER, file_name);
                                let color_image = self.load_image_from_path(&manga_panel_file_path);
                                if let Ok(manga_panel) = color_image {
                                    let texture = ui.ctx().load_texture(
                                        "manga_panel", // TODO: Probably need unique name here
                                        manga_panel.clone(),
                                        Default::default(),
                                    );
                                    if ui
                                        .button(Image::from_texture(&texture).max_height(300.0))
                                        .clicked()
                                    {
                                        ui.ctx().copy_image(manga_panel);
                                    }
                                }
                            }
                        });
                    });

                    if self.keywords_search_text.is_empty() && self.added_image_file_path.is_empty()
                    {
                        ctx.forget_all_images();
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
    }

    pub fn draw_central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.draw_add_manga_panel_modal(ctx);
            /*
            Initially I had put this under a condition and I was negating the
            condition when I was done showing the images, however that just made
            the images disappear super fast, since this is immediate mode UI
            */
            self.draw_manga_panels_gallery(ui, ctx);
        });
    }
}
