use eframe::egui::{
    self, Align, Button, Color32, CornerRadius, Frame, Layout, RichText, Stroke, TextEdit,
    TopBottomPanel, Ui, Visuals,
};

const TOP_PANEL_ELEMENTS_HEIGHT: f32 = 40.0;

#[derive(Default)]
pub struct MyApp {
    name: String,
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
                ui.add(Button::new(RichText::new("Add Image").color(Color32::BLACK)).frame(false));
            });
    }

    fn draw_top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let window_width = ui.available_rect_before_wrap().width();
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        self.draw_search_bar(ui, window_width * 0.50, "Enter keywords");
                        self.draw_search_bar(ui, window_width * 0.30, "Enter manga name");
                        self.draw_add_image_button(ui, window_width * 0.10);
                    })
                });
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
    }
}
