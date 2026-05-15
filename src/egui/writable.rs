use crate::egui::{ABBREVIATION, COMMON, IUPAC, names::Names};

use const_format::formatcp;
use egui::{InnerResponse, Response, Ui, Widget};
use egui_l20n::UiExt;
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
            if let Some(name) = ui.try_localize(&format!("{id}.abbreviation"))
                && ui.button((PENCIL, ABBREVIATION)).clicked()
            {
                text = name;
                changed = true;
            }
            if let Some(name) = ui.try_localize(&format!("{id}.common"))
                && ui.button((PENCIL, formatcp!("{COMMON} name"))).clicked()
            {
                text = name;
                changed = true;
            }
            if let Some(name) = ui.try_localize(&format!("{id}.iupac"))
                && ui.button((PENCIL, formatcp!("{IUPAC} name"))).clicked()
            {
                text = name;
                changed = true;
            }
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
