use eframe::egui::{Color32, CornerRadius, Frame, Response, Stroke, TextEdit, Ui};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::ui::constants::TOP_PANEL_ELEMENTS_HEIGHT;

pub fn draw_default_frame() -> Frame {
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

pub fn draw_search_bar(
    ui: &mut Ui,
    width: f32,
    search_text_to_edit: &mut String,
    default_text: &String,
    need_response: bool,
) -> Option<Response> {
    let text_edit_response = draw_default_frame()
        .fill(Color32::from_rgb(23, 23, 23))
        .show(ui, |ui| {
            ui.set_height(TOP_PANEL_ELEMENTS_HEIGHT);
            ui.add(
                TextEdit::singleline(search_text_to_edit)
                    .desired_width(width)
                    .hint_text(default_text)
                    .frame(false),
            );
        })
        .response;
    if need_response {
        Some(text_edit_response)
    } else {
        None
    }
}

pub fn get_manga_names_options(
    manga_names_list: &Vec<String>,
    add_manga_panel_modal_manga_name: &String,
) -> Vec<(i64, String)> {
    let matcher = SkimMatcherV2::default();
    let choices = manga_names_list;
    let mut manga_names_with_score: Vec<(i64, String)> = choices
        .iter()
        .filter_map(|item| {
            matcher
                .fuzzy_match(item, add_manga_panel_modal_manga_name)
                .map(|score| (score, item.clone()))
        })
        .collect();
    manga_names_with_score.sort_by(|a, b| b.0.cmp(&a.0));
    manga_names_with_score
}
