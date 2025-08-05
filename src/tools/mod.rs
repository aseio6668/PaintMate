pub mod brush;
pub mod pencil;
pub mod eraser;
pub mod fill;
pub mod rectangle;
pub mod circle;
pub mod line;
pub mod text;
pub mod crop;
pub mod eyedropper;

use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolType {
    Brush,
    Pencil,
    Eraser,
    Fill,
    Rectangle,
    Circle,
    Line,
    Text,
    Crop,
    Eyedropper,
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::Brush
    }
}

#[derive(Debug, Clone)]
pub struct Tool {
    pub tool_type: ToolType,
    pub is_active: bool,
}

impl Default for Tool {
    fn default() -> Self {
        Self {
            tool_type: ToolType::Brush,
            is_active: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BrushSettings {
    pub size: f32,
    pub opacity: f32,
    pub hardness: f32,
    pub spacing: f32,
    pub primary_color: egui::Color32,
    pub secondary_color: egui::Color32,
    pub border_width: f32,
    pub fill_enabled: bool,
    pub border_enabled: bool,
}

impl Default for BrushSettings {
    fn default() -> Self {
        Self {
            size: 10.0,
            opacity: 1.0,
            hardness: 1.0,
            spacing: 0.1,
            primary_color: egui::Color32::BLACK,
            secondary_color: egui::Color32::WHITE,
            border_width: 2.0,
            fill_enabled: true,
            border_enabled: true,
        }
    }
}

impl BrushSettings {
    pub fn show_ui(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Slider::new(&mut self.size, 1.0..=100.0)
                .text("Size")
                .suffix("px"),
        );
        
        ui.add(
            egui::Slider::new(&mut self.opacity, 0.0..=1.0)
                .text("Opacity"),
        );
        
        ui.add(
            egui::Slider::new(&mut self.hardness, 0.0..=1.0)
                .text("Hardness"),
        );
        
        ui.add(
            egui::Slider::new(&mut self.spacing, 0.01..=1.0)
                .text("Spacing"),
        );
        
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.label("Primary:");
            ui.color_edit_button_srgba(&mut self.primary_color);
        });
        
        ui.horizontal(|ui| {
            ui.label("Secondary:");
            ui.color_edit_button_srgba(&mut self.secondary_color);
        });
        
        ui.separator();
        
        ui.checkbox(&mut self.fill_enabled, "Fill");
        ui.checkbox(&mut self.border_enabled, "Border");
        
        if self.border_enabled {
            ui.add(
                egui::Slider::new(&mut self.border_width, 1.0..=20.0)
                    .text("Border Width")
                    .suffix("px"),
            );
        }
    }
}

pub trait ToolBehavior {
    fn start_stroke(&mut self, pos: egui::Pos2, settings: &BrushSettings);
    fn continue_stroke(&mut self, pos: egui::Pos2, settings: &BrushSettings);
    fn end_stroke(&mut self, settings: &BrushSettings);
    fn draw_preview(&self, ui: &mut egui::Ui, painter: &egui::Painter, pos: egui::Pos2, settings: &BrushSettings);
}

impl ToolType {
    pub fn name(&self) -> &'static str {
        match self {
            ToolType::Brush => "Brush",
            ToolType::Pencil => "Pencil",
            ToolType::Eraser => "Eraser",
            ToolType::Fill => "Fill",
            ToolType::Rectangle => "Rectangle",
            ToolType::Circle => "Circle",
            ToolType::Line => "Line",
            ToolType::Text => "Text",
            ToolType::Crop => "Crop",
            ToolType::Eyedropper => "Eyedropper",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            ToolType::Brush => "🖌",
            ToolType::Pencil => "✏",
            ToolType::Eraser => "🧹",
            ToolType::Fill => "🪣",
            ToolType::Rectangle => "⬜",
            ToolType::Circle => "⭕",
            ToolType::Line => "📏",
            ToolType::Text => "🔤",
            ToolType::Crop => "✂",
            ToolType::Eyedropper => "💧",
        }
    }
}
