use crate::db::{create_tables, start_connection};
use eframe::egui::{self, Color32, Visuals};
use rusqlite::Connection;

pub struct MyApp {
    pub keywords_search_text: String,
    pub manga_name_search_text: String,
    pub added_image_file_path: String,
    pub added_image_text: String,
    pub database_connection: Connection,
    pub add_manga_panel_modal_manga_name: String,
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
            add_manga_panel_modal_manga_name: Default::default(),
        }
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
