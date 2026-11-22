#![recursion_limit = "512"]

pub mod app;
pub mod auth;
pub mod brand;
pub mod components;
pub mod dashboard;
pub mod db;
pub mod i18n;
pub mod icons;
pub mod navigation;
pub mod preferences;
pub mod safe_area;
pub mod sync;
pub mod theme;
pub mod updater;
pub mod window;

pub fn mobile_entrypoint() {
    // Mobile hosts initialize GPUI and open UmuryangoApp via crates/mobile.
}
