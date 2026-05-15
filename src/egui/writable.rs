use crate::egui::names::Names;

use egui::{InnerResponse, Response, Ui, Widget};
use typed_builder::TypedBuilder;

/// Writable name widget
#[derive(TypedBuilder)]
pub struct Writable<'a> {
    id: &'a str,
    #[builder(default = true)]
    hover: bool,
    #[builder(default, setter(strip_option))]
    text: Option<&'a str>,
}

impl Writable<'_> {
    pub fn show(self, ui: &mut Ui) -> InnerResponse<Option<String>> {
        let mut text = self.text.unwrap_or_default();
        let mut response = ui.text_edit_singleline(&mut text);
        if response.changed() {
            return InnerResponse::new(Some(text.to_string()), response);
        }
        if self.hover {
            response = response.on_hover_ui(|ui| {
                Names::builder().id(self.id).build().ui(ui);
            });
        }
        InnerResponse::new(None, response)
    }
}

impl Widget for Writable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}
