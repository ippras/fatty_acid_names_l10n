#![feature(const_trait_impl)]
#![feature(const_cmp)]

pub const EN: &str = "en";
pub const RU: &str = "ru";

pub const ABBREVIATION: &str = "Abbreviation";
pub const COMMON: &str = "Common";
pub const ERASE: &str = "Erase";
pub const IUPAC: &str = "Iupac";
pub const NAME: &str = "Name";
pub const NAMES: &str = "Names";

pub const fn content(language: &str) -> &'static [&'static str] {
    match language {
        EN => &[
            asset!("/ftl/en/main.ftl"),
            asset!("/ftl/en/main.selectors.ftl"),
            asset!("/ftl/en/aocs.org.ftl"),
            asset!("/ftl/en/aocs.org.ext.ftl"),
        ],
        RU => &[
            asset!("/ftl/ru/main.ftl"),
            asset!("/ftl/en/aocs.org.ftl"),
            asset!("/ftl/en/aocs.org.ext.ftl"),
        ],
        _ => &[
            asset!("/ftl/en/aocs.org.ftl"),
            asset!("/ftl/en/aocs.org.ext.ftl"),
        ],
    }
}

#[cfg(feature = "egui")]
pub mod egui;

mod macros;
