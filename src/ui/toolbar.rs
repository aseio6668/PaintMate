use eframe::egui;
use crate::tools::{Tool, ToolType};

#[derive(Default)]
pub struct Toolbar;

impl Toolbar {
    pub fn show(&mut self, ui: &mut egui::Ui, current_tool: &mut Tool) {
        ui.vertical(|ui| {
            ui.heading("Tools");
            ui.separator();
            
            let tools = [
                ToolType::Brush,
                ToolType::Pencil,
                ToolType::Eraser,
                ToolType::Fill,
                ToolType::Rectangle,
                ToolType::Circle,
                ToolType::Line,
                ToolType::Text,
                ToolType::Crop,
                ToolType::Eyedropper,
            ];
            
            for tool_type in &tools {
                let is_selected = current_tool.tool_type == *tool_type;
                
                let button = egui::Button::new(format!("{} {}", tool_type.icon(), tool_type.name()))
                    .selected(is_selected)
                    .min_size(egui::Vec2::new(ui.available_width(), 30.0));
                
                if ui.add(button).clicked() {
                    current_tool.tool_type = tool_type.clone();
                }
            }
        });
    }
}
