use crate::egui::{ABBREVIATION, COMMON, IUPAC, names::Names};

use const_format::formatcp;
use egui::{Button, InnerResponse, Response, RichText, Ui, Widget};
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

            let abbreviation = ui.try_localize(&format!("{id}.abbreviation"));
            ui.add_enabled_ui(abbreviation.is_some(), |ui| {
                let mut atom = RichText::new(ABBREVIATION);
                if abbreviation.as_ref() == Some(&text) {
                    atom = atom.weak();
                }
                if ui.button((PENCIL, atom)).clicked()
                    && let Some(abbreviation) = abbreviation
                {
                    text = abbreviation;
                    changed = true;
                }
            });

            let common = ui.try_localize(&format!("{id}.common"));
            ui.add_enabled_ui(common.is_some(), |ui| {
                if ui.button((PENCIL, formatcp!("{COMMON} name"))).clicked()
                    && let Some(common) = common
                {
                    text = common;
                    changed = true;
                }
            });

            let iupac = ui.try_localize(&format!("{id}.iupac"));
            ui.add_enabled_ui(iupac.is_some(), |ui| {
                if ui.button((PENCIL, formatcp!("{IUPAC} name"))).clicked()
                    && let Some(iupac) = iupac
                {
                    text = iupac;
                    changed = true;
                }
            });

            if ui.button((ERASER, "Empty string")).clicked() {
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
