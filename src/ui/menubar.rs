use eframe::egui;
use crate::ui::app::{PaintMateApp, FileOperation};
use rfd::FileDialog;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub struct MenuBar {
    pub show_new_dialog: bool,
    pub new_width: String,
    pub new_height: String,
    pub show_about: bool,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self {
            show_new_dialog: false,
            new_width: "800".to_string(),
            new_height: "600".to_string(),
            show_about: false,
        }
    }
}

impl MenuBar {
    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut PaintMateApp) {
        let sender = app.get_file_operation_sender();
        self.show_with_sender(ui, app, sender);
    }

    pub fn show_with_sender(&mut self, ui: &mut egui::Ui, app: &mut PaintMateApp, sender: Sender<FileOperation>) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New (Ctrl+N)").clicked() {
                    self.show_new_dialog = true;
                    ui.close_menu();
                }
                
                if ui.button("Open (Ctrl+O)").clicked() {
                    self.request_open_file(sender.clone());
                    ui.close_menu();
                }
                
                ui.separator();
                
                let has_image = app.image_data.is_some();
                
                if ui.add_enabled(has_image, egui::Button::new("Save (Ctrl+S)")).clicked() {
                    if app.current_file.is_some() {
                        if let Err(e) = app.save_image(None) {
                            log::error!("Failed to save image: {}", e);
                        }
                    } else {
                        self.request_save_as(sender.clone());
                    }
                    ui.close_menu();
                }
                
                if ui.add_enabled(has_image, egui::Button::new("Save As (Ctrl+Shift+S)")).clicked() {
                    self.request_save_as(sender.clone());
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.add_enabled(has_image, egui::Button::new("Export")).clicked() {
                    self.request_export(sender.clone());
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Exit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            
            ui.menu_button("Edit", |ui| {
                let has_image = app.image_data.is_some();
                
                if ui.add_enabled(
                    has_image && app.image_history.can_undo(),
                    egui::Button::new("Undo (Ctrl+Z)")
                ).clicked() {
                    if let Some(previous_state) = app.image_history.undo() {
                        app.image_data = Some(previous_state.clone());
                    }
                    ui.close_menu();
                }
                
                if ui.add_enabled(
                    has_image && app.image_history.can_redo(),
                    egui::Button::new("Redo (Ctrl+Y)")
                ).clicked() {
                    if let Some(next_state) = app.image_history.redo() {
                        app.image_data = Some(next_state.clone());
                    }
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.add_enabled(has_image, egui::Button::new("Copy (Ctrl+C)")).clicked() {
                    self.copy_to_clipboard(app);
                    ui.close_menu();
                }
                
                if ui.add_enabled(has_image, egui::Button::new("Paste (Ctrl+V)")).clicked() {
                    self.paste_from_clipboard(app);
                    ui.close_menu();
                }
            });
            
            ui.menu_button("Image", |ui| {
                let has_image = app.image_data.is_some();
                
                if ui.add_enabled(has_image, egui::Button::new("Resize")).clicked() {
                    // TODO: Show resize dialog
                    ui.close_menu();
                }
                
                if ui.add_enabled(has_image, egui::Button::new("Crop")).clicked() {
                    app.current_tool.tool_type = crate::tools::ToolType::Crop;
                    ui.close_menu();
                }
                
                ui.separator();
                
                ui.menu_button("Rotate", |ui| {
                    if ui.add_enabled(has_image, egui::Button::new("90° CW")).clicked() {
                        self.rotate_image(app, 90);
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(has_image, egui::Button::new("180°")).clicked() {
                        self.rotate_image(app, 180);
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(has_image, egui::Button::new("90° CCW")).clicked() {
                        self.rotate_image(app, 270);
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Flip", |ui| {
                    if ui.add_enabled(has_image, egui::Button::new("Horizontal")).clicked() {
                        self.flip_image(app, true);
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(has_image, egui::Button::new("Vertical")).clicked() {
                        self.flip_image(app, false);
                        ui.close_menu();
                    }
                });
                
                ui.separator();
                
                ui.menu_button("Adjustments", |ui| {
                    if ui.add_enabled(has_image, egui::Button::new("Brightness/Contrast")).clicked() {
                        // TODO: Show brightness/contrast dialog
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(has_image, egui::Button::new("Hue/Saturation")).clicked() {
                        // TODO: Show hue/saturation dialog
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(has_image, egui::Button::new("Color Balance")).clicked() {
                        // TODO: Show color balance dialog
                        ui.close_menu();
                    }
                });
            });
            
            ui.menu_button("View", |ui| {
                if ui.button("Fullscreen (F11)").clicked() {
                    app.toggle_fullscreen();
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Zoom In (+)").clicked() {
                    app.zoom_level = (app.zoom_level * 1.2).min(10.0);
                    ui.close_menu();
                }
                
                if ui.button("Zoom Out (-)").clicked() {
                    app.zoom_level = (app.zoom_level / 1.2).max(0.1);
                    ui.close_menu();
                }
                
                if ui.button("Zoom to Fit").clicked() {
                    // TODO: Calculate zoom to fit
                    app.zoom_level = 1.0;
                    app.pan_offset = egui::Vec2::ZERO;
                    ui.close_menu();
                }
                
                if ui.button("Actual Size").clicked() {
                    app.zoom_level = 1.0;
                    app.pan_offset = egui::Vec2::ZERO;
                    ui.close_menu();
                }
            });
            
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    self.show_about = true;
                    ui.close_menu();
                }
            });
        });
        
        // Show dialogs
        self.show_new_image_dialog(ui.ctx(), app);
        self.show_about_dialog(ui.ctx());
    }
    
    pub fn request_open_file(&self, sender: Sender<FileOperation>) {
        std::thread::spawn(move || {
            if let Some(path) = FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"])
                .pick_file()
            {
                if let Err(e) = sender.send(FileOperation::Open(path)) {
                    log::error!("Failed to send open file operation: {}", e);
                }
            }
        });
    }
    
    pub fn request_save_as(&self, sender: Sender<FileOperation>) {
        std::thread::spawn(move || {
            if let Some(path) = FileDialog::new()
                .add_filter("PNG", &["png"])
                .add_filter("JPEG", &["jpg", "jpeg"])
                .add_filter("GIF", &["gif"])
                .add_filter("BMP", &["bmp"])
                .add_filter("TIFF", &["tiff"])
                .save_file()
            {
                if let Err(e) = sender.send(FileOperation::Save(path)) {
                    log::error!("Failed to send save file operation: {}", e);
                }
            }
        });
    }
    
    pub fn request_export(&self, sender: Sender<FileOperation>) {
        std::thread::spawn(move || {
            if let Some(path) = FileDialog::new()
                .add_filter("PNG", &["png"])
                .add_filter("JPEG", &["jpg", "jpeg"])
                .add_filter("TIFF", &["tiff"])
                .add_filter("BMP", &["bmp"])
                .save_file()
            {
                if let Err(e) = sender.send(FileOperation::Save(path)) {
                    log::error!("Failed to send export file operation: {}", e);
                }
            }
        });
    }
    
    fn show_new_image_dialog(&mut self, ctx: &egui::Context, app: &mut PaintMateApp) {
        if !self.show_new_dialog {
            return;
        }
        
        egui::Window::new("New Image")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.text_edit_singleline(&mut self.new_width);
                    ui.label("px");
                });
                
                ui.horizontal(|ui| {
                    ui.label("Height:");
                    ui.text_edit_singleline(&mut self.new_height);
                    ui.label("px");
                });
                
                ui.horizontal(|ui| {
                    if ui.button("Create").clicked() {
                        let width: u32 = self.new_width.parse().unwrap_or(800);
                        let height: u32 = self.new_height.parse().unwrap_or(600);
                        
                        app.new_image(width, height);
                        self.show_new_dialog = false;
                    }
                    
                    if ui.button("Cancel").clicked() {
                        self.show_new_dialog = false;
                    }
                });
            });
    }
    
    fn show_about_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_about {
            return;
        }
        
        egui::Window::new("About PaintMate")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("PaintMate");
                    ui.label("Version 0.1.0");
                    ui.separator();
                    ui.label("A cross-platform paint application");
                    ui.label("Built with Rust and egui");
                    ui.separator();
                    
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
            });
    }
    
    fn copy_to_clipboard(&self, app: &PaintMateApp) {
        // TODO: Implement clipboard copy
        log::info!("Copy to clipboard");
    }
    
    fn paste_from_clipboard(&self, app: &mut PaintMateApp) {
        // TODO: Implement clipboard paste
        log::info!("Paste from clipboard");
    }
    
    fn rotate_image(&self, app: &mut PaintMateApp, degrees: i32) {
        // TODO: Implement image rotation
        log::info!("Rotate image {} degrees", degrees);
    }
    
    fn flip_image(&self, app: &mut PaintMateApp, horizontal: bool) {
        // TODO: Implement image flipping
        log::info!("Flip image {}", if horizontal { "horizontally" } else { "vertically" });
    }
}
