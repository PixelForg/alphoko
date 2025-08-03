use eframe::egui::{Color32, CornerRadius, Frame, Response, Stroke, TextEdit, Ui};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

use crate::ui::constants::SIDE_PANEL_ELEMENTS_HEIGHT;

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
    search_text_to_edit: &mut String,
    default_text: &String,
    need_response: bool,
) -> Option<Response> {
    let text_edit_response = draw_default_frame()
        .fill(Color32::from_rgb(23, 23, 23))
        .show(ui, |ui| {
            ui.set_height(SIDE_PANEL_ELEMENTS_HEIGHT);
            ui.add(
                TextEdit::singleline(search_text_to_edit)
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

pub fn get_fuzzy_search_options(
    options: &Vec<String>,
    search_query: &String,
) -> Vec<(i64, String)> {
    let matcher = SkimMatcherV2::default().ignore_case();
    let choices = options;
    let mut manga_names_with_score: Vec<(i64, String)> = choices
        .iter()
        .filter_map(|item| {
            matcher
                .fuzzy_match(item, search_query)
                .map(|score| (score, item.clone()))
        })
        .collect();
    manga_names_with_score.sort_by(|a, b| b.0.cmp(&a.0));
    manga_names_with_score
}
