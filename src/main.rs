mod app;
mod db;
mod ui {
    pub mod central_panel;
    pub mod common;
    pub mod constants;
    pub mod side_panel;
}

use crate::app::MyApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 960.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Alphoko - A fast manga panel searcher",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}
