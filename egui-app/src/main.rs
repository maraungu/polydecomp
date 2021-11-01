// Forbid warnings in release builds:
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
fn main() {
    let app = egui_lib::app::DecompApp::default();
    let options = eframe::NativeOptions::default(); //{
        // Let's show off that we support transparent windows
        //transparent: false,
        //drag_and_drop_support: true,
        //..Default::default()
    //};
    eframe::run_native(Box::new(app), options);
}
