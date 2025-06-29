use std::f32;

use eframe::egui::{self, CornerRadius, Margin, Stroke};
use egui::{Color32, Frame, TextEdit, Visuals};

struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals {
            panel_fill: Color32::BLACK,
            override_text_color: Some(Color32::WHITE),
            ..Default::default()
        });
        egui::TopBottomPanel::top("my_panel")
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

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 960.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Alphoko - A fast manga panel searcher",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}
