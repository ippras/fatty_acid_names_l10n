#![feature(const_trait_impl)]
#![feature(const_cmp)]

pub const EN: &str = "en";
pub const RU: &str = "ru";

pub const fn content(language: &str) -> &'static [&'static str] {
    match language {
        EN => &[
            asset!("/ftl/en/main.ftl"),
            asset!("/ftl/en/aocs.org.ftl"),
            asset!("/ftl/en/aocs.org.ext.ftl"),
        ],
        RU => &[asset!("/ftl/en/main.ftl"), asset!("/ftl/ru/aocs.org.ftl")],
        _ => &[asset!("/ftl/en/aocs.org.ftl")],
    }
}

#[cfg(feature = "egui")]
pub mod egui;

mod macros;
