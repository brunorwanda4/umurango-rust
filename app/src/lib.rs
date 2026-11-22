#![recursion_limit = "2048"]

pub mod app;
pub mod auth;
pub mod brand;
pub mod campaigns;
pub mod components;
pub mod dashboard;
pub mod db;
pub mod events;
pub mod home;
pub mod i18n;
pub mod icons;
pub mod members;
pub mod money;
pub mod navigation;
pub mod preferences;
pub mod projects;
pub mod reports;
pub mod safe_area;
pub mod settings;
pub mod sync;
pub mod theme;
pub mod updater;
pub mod vault;
pub mod window;

pub fn mobile_entrypoint() {
    // Mobile hosts initialize GPUI and open UmuryangoApp via crates/mobile.
}
