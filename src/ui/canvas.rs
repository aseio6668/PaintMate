use eframe::egui;
use crate::image_ops::ImageData;
use crate::tools::{Tool, BrushSettings};

pub struct CanvasState {
    pub is_drawing: bool,
    pub last_pos: Option<egui::Pos2>,
    pub current_stroke: Vec<egui::Pos2>,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            is_drawing: false,
            last_pos: None,
            current_stroke: Vec::new(),
        }
    }
}

impl CanvasState {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        image_data: &mut Option<ImageData>,
        current_tool: &mut Tool,
        brush_settings: &BrushSettings,
        zoom_level: &mut f32,
        pan_offset: &mut egui::Vec2,
        is_modified: &mut bool,
    ) {
        let available_rect = ui.available_rect_before_wrap();
        
        if let Some(ref mut img_data) = image_data {
            let image_size = egui::Vec2::new(img_data.width() as f32, img_data.height() as f32);
            let scaled_size = image_size * *zoom_level;
            
            // Center the image in the available space
            let center_offset = (available_rect.size() - scaled_size) * 0.5;
            let image_rect = egui::Rect::from_min_size(
                available_rect.min + center_offset + *pan_offset,
                scaled_size,
            );
            
            // Handle input
            let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());
            
            // Handle zoom
            if response.hovered() {
                let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
                if scroll_delta != 0.0 {
                    let old_zoom = *zoom_level;
                    *zoom_level = (*zoom_level * (1.0 + scroll_delta * 0.001)).clamp(0.1, 10.0);
                    
                    // Adjust pan to zoom towards mouse position
                    if let Some(hover_pos) = response.hover_pos() {
                        let relative_pos = hover_pos - image_rect.center();
                        let zoom_factor = *zoom_level / old_zoom;
                        *pan_offset += relative_pos * (1.0 - zoom_factor);
                    }
                }
            }
            
            // Handle panning with middle mouse or space+drag
            if response.dragged_by(egui::PointerButton::Middle) 
                || (ui.input(|i| i.key_down(egui::Key::Space)) && response.dragged()) {
                *pan_offset += response.drag_delta();
            }
            
            // Handle drawing
            if response.hovered() && ui.input(|i| i.pointer.primary_down()) {
                if let Some(hover_pos) = response.hover_pos() {
                    if image_rect.contains(hover_pos) {
                        let relative_pos = hover_pos - image_rect.min;
                        let image_pos = relative_pos / *zoom_level;
                        let image_pos_point = egui::Pos2::new(image_pos.x, image_pos.y);
                        let pixel_x = image_pos.x as u32;
                        let pixel_y = image_pos.y as u32;
                        
                        if !self.is_drawing {
                            self.is_drawing = true;
                            self.current_stroke.clear();
                            self.current_stroke.push(image_pos_point);
                        } else {
                            self.current_stroke.push(image_pos_point);
                        }
                        
                        // Draw based on current tool
                        self.apply_tool(img_data, pixel_x, pixel_y, brush_settings, current_tool);
                        *is_modified = true;
                        
                        self.last_pos = Some(image_pos_point);
                    }
                }
            } else if self.is_drawing {
                self.is_drawing = false;
                self.last_pos = None;
                self.current_stroke.clear();
            }
            
            // Draw the image
            let texture = img_data.get_texture(ui.ctx());
            ui.painter().image(
                texture.id(),
                image_rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
            
            // Draw border around image
            ui.painter().rect_stroke(
                image_rect,
                0.0,
                egui::Stroke::new(1.0, egui::Color32::GRAY),
            );
            
            // Draw cursor preview
            if let Some(hover_pos) = response.hover_pos() {
                if image_rect.contains(hover_pos) {
                    self.draw_cursor_preview(ui, hover_pos, brush_settings);
                }
            }
        } else {
            // No image loaded - show welcome message
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Welcome to PaintMate");
                    ui.label("Create a new image or open an existing one to get started");
                    
                    ui.separator();
                    
                    if ui.button("New Image (Ctrl+N)").clicked() {
                        // This will be handled by the menu bar
                    }
                    
                    if ui.button("Open Image (Ctrl+O)").clicked() {
                        // This will be handled by the menu bar
                    }
                });
            });
        }
    }
    
    fn apply_tool(&mut self, image_data: &mut ImageData, x: u32, y: u32, brush_settings: &BrushSettings, current_tool: &Tool) {
        let color = image::Rgba([
            brush_settings.primary_color.r(),
            brush_settings.primary_color.g(),
            brush_settings.primary_color.b(),
            (brush_settings.primary_color.a() as f32 * brush_settings.opacity) as u8,
        ]);
        
        match current_tool.tool_type {
            crate::tools::ToolType::Brush | crate::tools::ToolType::Pencil => {
                // Draw a circle for brush/pencil
                image_data.draw_circle(x, y, brush_settings.size / 2.0, color);
            }
            crate::tools::ToolType::Eraser => {
                // Draw with transparent color
                let transparent = image::Rgba([0, 0, 0, 0]);
                image_data.draw_circle(x, y, brush_settings.size / 2.0, transparent);
            }
            _ => {
                // For other tools, just draw a pixel for now
                image_data.draw_pixel(x, y, color);
            }
        }
    }
    
    fn draw_cursor_preview(&self, ui: &mut egui::Ui, pos: egui::Pos2, brush_settings: &BrushSettings) {
        let painter = ui.painter();
        let radius = brush_settings.size / 2.0;
        
        painter.circle_stroke(
            pos,
            radius,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
        
        painter.circle_stroke(
            pos,
            radius,
            egui::Stroke::new(1.0, egui::Color32::BLACK),
        );
    }
}
