#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod tools;
mod image_ops;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "PaintMate",
        options,
        Box::new(|cc| {
            // Setup custom fonts if available
            setup_custom_fonts(&cc.egui_ctx);
            
            // Enable dark mode by default
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            
            Box::new(ui::PaintMateApp::new(cc))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // We'll use default fonts for now
    // TODO: Add custom fonts when available
    
    ctx.set_fonts(fonts);
}
