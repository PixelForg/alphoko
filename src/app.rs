use crate::db::{
    create_tables, retrieve_manga_names_from_db, retrieve_manga_panels_text_from_db,
    start_connection,
};
use eframe::egui::{self, Color32, Visuals};
use rusqlite::Connection;

pub struct MyApp {
    pub keywords_search_text: String,
    pub manga_name_search_text: String,
    pub added_image_file_path: String,
    pub added_image_text: String,
    pub database_connection: Connection,
    pub add_manga_panel_modal_manga_name: String,
    pub manga_names_list: Vec<String>,
    pub manga_panels_text_list: Vec<String>,
    pub manga_keyword_was_chosen: bool,
    pub manga_name_was_chosen: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let connection = start_connection().expect("Failed to start connection");
        create_tables(&connection).expect("Failed to create tables");
        let manga_names_list = retrieve_manga_names_from_db(&connection);
        let manga_panels_text_list = retrieve_manga_panels_text_from_db(&connection);
        Self {
            keywords_search_text: Default::default(),
            manga_name_search_text: Default::default(),
            added_image_file_path: Default::default(),
            added_image_text: Default::default(),
            database_connection: connection,
            add_manga_panel_modal_manga_name: Default::default(),
            manga_names_list: manga_names_list.unwrap_or(Default::default()),
            manga_panels_text_list: manga_panels_text_list.unwrap_or(Default::default()),
            manga_keyword_was_chosen: false,
            manga_name_was_chosen: false,
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
