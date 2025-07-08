use crate::db::{add_manga_panel_to_db, create_tables, start_connection};
use eframe::egui::{
    self, Align, Button, CentralPanel, Color32, CornerRadius, Frame, Image, Layout, Modal,
    RichText, Stroke, TextEdit, TopBottomPanel, Ui, Vec2, Visuals,
};
use rfd::FileDialog;
use rusqlite::Connection;
use std::fs;

const TOP_PANEL_ELEMENTS_HEIGHT: f32 = 40.0;
const MANGA_PANELS_SAVE_FOLDER: &str = "/home/pixelforg/Pictures/alphoko"; // TODO Remove this

enum SearchBarText {
    Keywords(String),
    MangaName(String),
}

pub struct MyApp {
    keywords_search_text: String,
    manga_name_search_text: String,
    added_image_file_path: String,
    added_image_text: String,
    database_connection: Connection,
}

impl Default for MyApp {
    fn default() -> Self {
        let connection = start_connection().expect("Failed to start connection");
        create_tables(&connection).expect("Failed to create tables");
        Self {
            keywords_search_text: Default::default(),
            manga_name_search_text: Default::default(),
            added_image_file_path: Default::default(),
            added_image_text: Default::default(),
            database_connection: connection,
        }
    }
}

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
                ui.set_height(TOP_PANEL_ELEMENTS_HEIGHT);
                if ui
                    .add(
                        Button::new(RichText::new("Upload manga panel").color(Color32::BLACK))
                            .frame(false),
                    )
                    .clicked()
                {
                    if let Some(added_image_file_path) = FileDialog::new().pick_file() {
                        self.added_image_file_path = added_image_file_path.display().to_string();
                    }
                }
            });
    }

    fn draw_top_panel(&mut self, ctx: &egui::Context) {
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
                            self.added_image_file_path.clear();
                            self.added_image_text.clear();
                            ctx.forget_image(&uri);
                        }
                    });
                    if ui.button("Close without saving").clicked() {
                        self.added_image_file_path.clear();
                        self.added_image_text.clear();
                        ctx.forget_image(&uri);
                    }
                });
            });
        }
    }

    fn draw_central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.draw_add_manga_panel_modal(ctx);
            ui.label(&self.keywords_search_text);
            ui.label(&self.manga_name_search_text);
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals {
            panel_fill: Color32::from_rgb(0, 0, 0),
            override_text_color: Some(Color32::WHITE),
            ..Default::default()
        });
        self.draw_top_panel(ctx);
        self.draw_central_panel(ctx);
    }
}
