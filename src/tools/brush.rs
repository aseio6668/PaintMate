// Basic brush tool implementation
use crate::tools::ToolBehavior;
use crate::tools::BrushSettings;
use eframe::egui;

pub struct BrushTool {
    pub current_stroke: Vec<egui::Pos2>,
}

impl Default for BrushTool {
    fn default() -> Self {
        Self {
            current_stroke: Vec::new(),
        }
    }
}

impl ToolBehavior for BrushTool {
    fn start_stroke(&mut self, pos: egui::Pos2, _settings: &BrushSettings) {
        self.current_stroke.clear();
        self.current_stroke.push(pos);
    }
    
    fn continue_stroke(&mut self, pos: egui::Pos2, _settings: &BrushSettings) {
        self.current_stroke.push(pos);
    }
    
    fn end_stroke(&mut self, _settings: &BrushSettings) {
        self.current_stroke.clear();
    }
    
    fn draw_preview(&self, _ui: &mut egui::Ui, painter: &egui::Painter, pos: egui::Pos2, settings: &BrushSettings) {
        let radius = settings.size / 2.0;
        painter.circle_stroke(
            pos,
            radius,
            egui::Stroke::new(1.0, egui::Color32::from_black_alpha(128)),
        );
    }
}
