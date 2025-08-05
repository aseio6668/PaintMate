use eframe::egui;
use crate::tools::{Tool, ToolType, BrushSettings};
use crate::image_ops::{ImageData, ImageHistory};
use crate::ui::{canvas::CanvasState, toolbar::Toolbar, menubar::MenuBar, color_picker::ColorPicker, layer_panel::LayerPanel};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use anyhow::Result;

#[derive(Debug)]
pub enum FileOperation {
    Open(PathBuf),
    Save(PathBuf),
}

pub struct PaintMateApp {
    pub canvas_state: CanvasState,
    pub current_tool: Tool,
    pub brush_settings: BrushSettings,
    pub toolbar: Toolbar,
    pub menubar: MenuBar,
    pub color_picker: ColorPicker,
    pub layer_panel: LayerPanel,
    pub image_data: Option<ImageData>,
    pub image_history: ImageHistory,
    pub current_file: Option<PathBuf>,
    pub is_fullscreen: bool,
    pub fullscreen_background: FullscreenBackground,
    pub show_ui: bool,
    pub zoom_level: f32,
    pub pan_offset: egui::Vec2,
    pub is_modified: bool,
    pub file_op_receiver: Receiver<FileOperation>,
    pub file_op_sender: Sender<FileOperation>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FullscreenBackground {
    Black,
    Alpha,
    White,
}

impl Default for PaintMateApp {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            canvas_state: CanvasState::default(),
            current_tool: Tool::default(),
            brush_settings: BrushSettings::default(),
            toolbar: Toolbar::default(),
            menubar: MenuBar::default(),
            color_picker: ColorPicker::default(),
            layer_panel: LayerPanel::default(),
            image_data: None,
            image_history: ImageHistory::new(),
            current_file: None,
            is_fullscreen: false,
            fullscreen_background: FullscreenBackground::Black,
            show_ui: true,
            zoom_level: 1.0,
            pan_offset: egui::Vec2::ZERO,
            is_modified: false,
            file_op_receiver: receiver,
            file_op_sender: sender,
        }
    }
}

impl PaintMateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn new_image(&mut self, width: u32, height: u32) {
        self.image_data = Some(ImageData::new(width, height));
        self.image_history.clear();
        self.current_file = None;
        self.is_modified = false;
        self.zoom_level = 1.0;
        self.pan_offset = egui::Vec2::ZERO;
    }

    pub fn open_image(&mut self, path: PathBuf) -> Result<()> {
        let image_data = ImageData::from_file(&path)?;
        self.image_data = Some(image_data);
        self.image_history.clear();
        self.current_file = Some(path);
        self.is_modified = false;
        self.zoom_level = 1.0;
        self.pan_offset = egui::Vec2::ZERO;
        Ok(())
    }

    pub fn save_image(&mut self, path: Option<PathBuf>) -> Result<()> {
        if let Some(ref image_data) = self.image_data {
            let save_path = match path {
                Some(p) => {
                    self.current_file = Some(p.clone());
                    p
                },
                None => {
                    if let Some(ref current_path) = self.current_file {
                        current_path.clone()
                    } else {
                        return Err(anyhow::anyhow!("No file path specified"));
                    }
                }
            };
            
            image_data.save_to_file(&save_path)?;
            self.is_modified = false;
        }
        Ok(())
    }

    pub fn get_file_operation_sender(&self) -> Sender<FileOperation> {
        self.file_op_sender.clone()
    }

    pub fn process_file_operations(&mut self) {
        while let Ok(operation) = self.file_op_receiver.try_recv() {
            match operation {
                FileOperation::Open(path) => {
                    if let Err(e) = self.open_image(path) {
                        log::error!("Failed to open image: {}", e);
                    }
                }
                FileOperation::Save(path) => {
                    if let Err(e) = self.save_image(Some(path)) {
                        log::error!("Failed to save image: {}", e);
                    }
                }
            }
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        self.is_fullscreen = !self.is_fullscreen;
        self.show_ui = !self.is_fullscreen;
    }

    pub fn cycle_fullscreen_background(&mut self) {
        self.fullscreen_background = match self.fullscreen_background {
            FullscreenBackground::Black => FullscreenBackground::Alpha,
            FullscreenBackground::Alpha => FullscreenBackground::White,
            FullscreenBackground::White => FullscreenBackground::Black,
        };
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::F11)) {
            self.toggle_fullscreen();
        }
        
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::N)) {
            // Show new image dialog
            self.menubar.show_new_dialog = true;
        }
        
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::O)) {
            self.menubar.request_open_file(self.file_op_sender.clone());
        }
        
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
            if self.current_file.is_some() {
                if let Err(e) = self.save_image(None) {
                    log::error!("Failed to save image: {}", e);
                }
            } else {
                self.menubar.request_save_as(self.file_op_sender.clone());
            }
        }
        
        if ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::S)) {
            self.menubar.request_save_as(self.file_op_sender.clone());
        }
        
        if self.is_fullscreen {
            if ctx.input(|i| i.key_pressed(egui::Key::Tab)) {
                self.show_ui = !self.show_ui;
            }
            
            if ctx.input(|i| i.key_pressed(egui::Key::B)) {
                self.cycle_fullscreen_background();
            }
        }
    }
}

impl eframe::App for PaintMateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process file operations first
        self.process_file_operations();
        
        self.handle_shortcuts(ctx);
        
        if self.is_fullscreen {
            self.update_fullscreen(ctx);
        } else {
            self.update_windowed(ctx);
        }
    }
}

impl PaintMateApp {
    fn update_windowed(&mut self, ctx: &egui::Context) {
        // Get a copy of the sender to avoid borrowing issues
        let file_sender = self.file_op_sender.clone();
        
        // Menu bar
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New (Ctrl+N)").clicked() {
                        self.menubar.show_new_dialog = true;
                        ui.close_menu();
                    }
                    
                    if ui.button("Open (Ctrl+O)").clicked() {
                        self.menubar.request_open_file(file_sender.clone());
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    let has_image = self.image_data.is_some();
                    
                    if ui.add_enabled(has_image, egui::Button::new("Save (Ctrl+S)")).clicked() {
                        if self.current_file.is_some() {
                            if let Err(e) = self.save_image(None) {
                                log::error!("Failed to save image: {}", e);
                            }
                        } else {
                            self.menubar.request_save_as(file_sender.clone());
                        }
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(has_image, egui::Button::new("Save As (Ctrl+Shift+S)")).clicked() {
                        self.menubar.request_save_as(file_sender.clone());
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.add_enabled(has_image, egui::Button::new("Export")).clicked() {
                        self.menubar.request_export(file_sender.clone());
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    let has_image = self.image_data.is_some();
                    
                    if ui.add_enabled(
                        has_image && self.image_history.can_undo(),
                        egui::Button::new("Undo (Ctrl+Z)")
                    ).clicked() {
                        if let Some(previous_state) = self.image_history.undo() {
                            self.image_data = Some(previous_state.clone());
                        }
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(
                        has_image && self.image_history.can_redo(),
                        egui::Button::new("Redo (Ctrl+Y)")
                    ).clicked() {
                        if let Some(next_state) = self.image_history.redo() {
                            self.image_data = Some(next_state.clone());
                        }
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.add_enabled(has_image, egui::Button::new("Copy (Ctrl+C)")).clicked() {
                        // TODO: Implement clipboard copy
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(has_image, egui::Button::new("Paste (Ctrl+V)")).clicked() {
                        // TODO: Implement clipboard paste
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.menubar.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
        
        // Show dialogs
        self.show_dialogs(ctx);

        // Tool bar
        egui::SidePanel::left("toolbar")
            .min_width(60.0)
            .max_width(80.0)
            .show(ctx, |ui| {
                self.toolbar.show(ui, &mut self.current_tool);
            });

        // Color picker and brush settings
        egui::SidePanel::right("properties")
            .min_width(200.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Colors");
                self.color_picker.show(ui);
                
                // Sync colors between color picker and brush settings
                self.brush_settings.primary_color = self.color_picker.primary_color;
                self.brush_settings.secondary_color = self.color_picker.secondary_color;
                
                ui.separator();
                
                ui.heading("Brush Settings");
                self.brush_settings.show_ui(ui);
                
                ui.separator();
                
                ui.heading("Layers");
                self.layer_panel.show(ui, &mut self.image_data);
            });

        // Status bar
        egui::TopBottomPanel::bottom("statusbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(ref image_data) = self.image_data {
                    ui.label(format!("{}x{}", image_data.width(), image_data.height()));
                    ui.separator();
                }
                
                ui.label(format!("Zoom: {:.0}%", self.zoom_level * 100.0));
                ui.separator();
                
                if self.is_modified {
                    ui.label("Modified");
                    ui.separator();
                }
                
                if let Some(ref path) = self.current_file {
                    ui.label(path.file_name().unwrap_or_default().to_string_lossy());
                } else {
                    ui.label("Untitled");
                }
            });
        });

        // Main canvas area
        egui::CentralPanel::default().show(ctx, |ui| {
            self.canvas_state.show(
                ui,
                &mut self.image_data,
                &mut self.current_tool,
                &self.brush_settings,
                &mut self.zoom_level,
                &mut self.pan_offset,
                &mut self.is_modified,
            );
        });
    }

    fn update_fullscreen(&mut self, ctx: &egui::Context) {
        // Set background based on fullscreen background setting
        let bg_color = match self.fullscreen_background {
            FullscreenBackground::Black => egui::Color32::BLACK,
            FullscreenBackground::White => egui::Color32::WHITE,
            FullscreenBackground::Alpha => egui::Color32::TRANSPARENT,
        };
        
        ctx.style_mut(|style| {
            style.visuals.panel_fill = bg_color;
            style.visuals.extreme_bg_color = bg_color;
        });

        if self.show_ui {
            // Minimal UI in fullscreen
            egui::TopBottomPanel::top("fullscreen_controls")
                .frame(egui::Frame::none().fill(egui::Color32::from_black_alpha(128)))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Exit Fullscreen (F11)").clicked() {
                            self.toggle_fullscreen();
                        }
                        
                        ui.separator();
                        
                        if ui.button("Toggle UI (Tab)").clicked() {
                            self.show_ui = false;
                        }
                        
                        ui.separator();
                        
                        if ui.button("Background (B)").clicked() {
                            self.cycle_fullscreen_background();
                        }
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("Zoom: {:.0}%", self.zoom_level * 100.0));
                        });
                    });
                });
        }

        // Main canvas area
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(bg_color))
            .show(ctx, |ui| {
                self.canvas_state.show(
                    ui,
                    &mut self.image_data,
                    &mut self.current_tool,
                    &self.brush_settings,
                    &mut self.zoom_level,
                    &mut self.pan_offset,
                    &mut self.is_modified,
                );
            });
    }

    fn show_dialogs(&mut self, ctx: &egui::Context) {
        // Show new image dialog
        if self.menubar.show_new_dialog {
            egui::Window::new("New Image")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Width:");
                        ui.text_edit_singleline(&mut self.menubar.new_width);
                        ui.label("px");
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Height:");
                        ui.text_edit_singleline(&mut self.menubar.new_height);
                        ui.label("px");
                    });
                    
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let width: u32 = self.menubar.new_width.parse().unwrap_or(800);
                            let height: u32 = self.menubar.new_height.parse().unwrap_or(600);
                            
                            self.new_image(width, height);
                            self.menubar.show_new_dialog = false;
                        }
                        
                        if ui.button("Cancel").clicked() {
                            self.menubar.show_new_dialog = false;
                        }
                    });
                });
        }
        
        // Show about dialog
        if self.menubar.show_about {
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
                            self.menubar.show_about = false;
                        }
                    });
                });
        }
    }
}
