use eframe::egui::{
    self, Color32, CornerRadius, Frame, Margin, Stroke, TextEdit, TopBottomPanel, Visuals,
};

#[derive(Default)]
pub struct MyApp {
    name: String,
}

impl MyApp {
    fn draw_search_bar(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .frame(Frame {
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::WHITE,
                },
                outer_margin: Margin {
                    left: 50,
                    right: 50,
                    top: 10,
                    bottom: 10,
                },
                corner_radius: CornerRadius {
                    nw: 1,
                    ne: 1,
                    sw: 1,
                    se: 1,
                },
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.add(
                    TextEdit::singleline(&mut self.name)
                        .desired_width(f32::INFINITY)
                        .margin(Margin {
                            left: 10,
                            right: 10,
                            top: 10,
                            bottom: 10,
                        })
                        .frame(false),
                );
            });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals {
            panel_fill: Color32::BLACK,
            override_text_color: Some(Color32::WHITE),
            ..Default::default()
        });
        self.draw_search_bar(ctx);
    }
}
