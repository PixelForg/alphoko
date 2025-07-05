use eframe::egui::{
    self, Align, Button, CentralPanel, Color32, CornerRadius, Frame, Image, Layout, Modal,
    RichText, Stroke, TextEdit, TopBottomPanel, Ui, Visuals, include_image, popup,
};
use rfd::FileDialog;

const TOP_PANEL_ELEMENTS_HEIGHT: f32 = 40.0;

#[derive(Default)]
pub struct MyApp {
    name: String,
    file_path: String,
}

impl MyApp {
    fn draw_search_bar(&mut self, ui: &mut Ui, width: f32, default_text: &str) {
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
            .fill(Color32::from_rgb(23, 23, 23))
            .show(ui, |ui| {
                ui.set_height(TOP_PANEL_ELEMENTS_HEIGHT);
                ui.add(
                    TextEdit::singleline(&mut self.name)
                        .desired_width(width)
                        .hint_text(default_text)
                        .frame(false),
                );
            });
    }

    fn draw_add_image_button(&mut self, ui: &mut Ui, width: f32) {
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
                    if let Some(file_path) = FileDialog::new().pick_file() {
                        self.file_path = file_path.display().to_string();
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
                        self.draw_search_bar(ui, window_width * 0.50, "Enter search text");
                        self.draw_search_bar(ui, window_width * 0.30, "Enter manga name");
                        self.draw_add_image_button(ui, window_width * 0.10);
                    })
                });
                ui.label(&self.file_path);
            });
    }
    fn draw_central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            if !self.file_path.is_empty() {
                Modal::new("modal".into()).show(ctx, |ui| {
                    ui.set_width(300.0);
                    ui.vertical_centered(|ui| ui.heading("Add image"));
                    // ui.heading("Add image");
                    ui.label("ello world");
                    let image = Image::new(format!("file://{}", &self.file_path));
                    ui.add(image)
                });
            }
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
