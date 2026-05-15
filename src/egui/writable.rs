use crate::egui::{ABBREVIATION, COMMON, EMPTY, IUPAC, NAME, names::Names};

use const_format::formatcp;
use egui::{InnerResponse, Response, RichText, Ui, Widget};
use egui_l20n::UiExt as _;
use egui_phosphor::regular::{ERASER, PENCIL};
use typed_builder::TypedBuilder;

/// Writable name widget
#[derive(TypedBuilder)]
pub struct Writable<'a> {
    id: &'a str,
    #[builder(default = true)]
    hover: bool,
    #[builder(default, setter(strip_option))]
    text: Option<String>,
}

impl Writable<'_> {
    pub fn show(self, ui: &mut Ui) -> InnerResponse<String> {
        let mut text = self.text.unwrap_or_default();
        let mut response = ui.text_edit_singleline(&mut text);
        if response.changed() {
            return InnerResponse::new(text, response);
        }
        let mut changed = false;
        response.context_menu(|ui| {
            let id = self.id;

            let abbreviation = ui.try_localize(&format!("{id}.{}", ABBREVIATION.to_lowercase()));
            ui.add_enabled_ui(abbreviation.is_some(), |ui| {
                let mut atom = RichText::new(ui.localize(ABBREVIATION));
                if matches!(&abbreviation, Some(abbreviation) if &text == abbreviation) {
                    atom = atom.strong();
                }
                if ui.button((PENCIL, atom)).clicked()
                    && let Some(abbreviation) = abbreviation
                {
                    text = abbreviation;
                    changed = true;
                }
            });

            let common = ui.try_localize(&format!("{id}.{}", COMMON.to_lowercase()));
            ui.add_enabled_ui(common.is_some(), |ui| {
                let mut atom = RichText::new(ui.localize(formatcp!("{COMMON}{NAME}")));
                if matches!(&common, Some(common) if &text == common) {
                    atom = atom.strong();
                }
                if ui.button((PENCIL, atom)).clicked()
                    && let Some(common) = common
                {
                    text = common;
                    changed = true;
                }
            });

            let iupac = ui.try_localize(&format!("{id}.{}", IUPAC.to_lowercase()));
            ui.add_enabled_ui(iupac.is_some(), |ui| {
                let mut atom = RichText::new(ui.localize(formatcp!("{IUPAC}{NAME}")));
                if matches!(&iupac, Some(iupac) if &text == iupac) {
                    atom = atom.strong();
                }
                if ui.button((PENCIL, atom)).clicked()
                    && let Some(iupac) = iupac
                {
                    text = iupac;
                    changed = true;
                }
            });

            if ui.button((ERASER, ui.localize(EMPTY))).clicked() {
                text = String::new();
                changed = true;
            }
        });
        if changed {
            response.mark_changed();
            return InnerResponse::new(text, response);
        }
        if self.hover {
            response = response.on_hover_ui(|ui| {
                Names::builder().id(self.id).build().ui(ui);
            });
        }
        InnerResponse::new(text, response)
    }
}

impl Widget for Writable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}

// fn button(ui: &mut Ui, id: &str, attribute: &str, source: &str) -> Option<String> {
//     let target = ui.try_localize(&format!("{id}.{}", attribute.to_lowercase()));
//     ui.add_enabled_ui(target.is_some(), |ui| {
//         let mut atom = RichText::new(ui.localize(attribute));
//         if matches!(&target, Some(target) if source == target) {
//             atom = atom.strong();
//         }
//         if ui.button((PENCIL, atom)).clicked() {
//             return target;
//         }
//         None
//     })
//     .inner
// }
