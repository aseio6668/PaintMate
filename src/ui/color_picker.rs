use eframe::egui;

#[derive(Default)]
pub struct ColorPicker {
    pub primary_color: egui::Color32,
    pub secondary_color: egui::Color32,
    pub show_advanced: bool,
}

impl ColorPicker {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Primary and secondary color display
        ui.horizontal(|ui| {
            // Primary color (larger)
            let primary_size = egui::Vec2::new(40.0, 40.0);
            let (primary_rect, _primary_response) = ui.allocate_exact_size(primary_size, egui::Sense::click());
            ui.painter().rect_filled(primary_rect, 2.0, self.primary_color);
            ui.painter().rect_stroke(primary_rect, 2.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
            
            ui.vertical(|ui| {
                // Secondary color (smaller, offset)
                let secondary_size = egui::Vec2::new(25.0, 25.0);
                let (secondary_rect, _secondary_response) = ui.allocate_exact_size(secondary_size, egui::Sense::click());
                ui.painter().rect_filled(secondary_rect, 2.0, self.secondary_color);
                ui.painter().rect_stroke(secondary_rect, 2.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
                
                // Swap button
                if ui.small_button("⇄").clicked() {
                    std::mem::swap(&mut self.primary_color, &mut self.secondary_color);
                }
            });
        });
        
        ui.separator();
        
        // Color editor
        ui.color_edit_button_srgba(&mut self.primary_color);
        ui.label("Primary");
        
        ui.color_edit_button_srgba(&mut self.secondary_color);
        ui.label("Secondary");
        
        ui.separator();
        
        // Predefined colors palette
        ui.label("Palette:");
        let colors = [
            egui::Color32::BLACK,
            egui::Color32::WHITE,
            egui::Color32::RED,
            egui::Color32::GREEN,
            egui::Color32::BLUE,
            egui::Color32::YELLOW,
            egui::Color32::LIGHT_BLUE,
            egui::Color32::LIGHT_GREEN,
            egui::Color32::LIGHT_RED,
            egui::Color32::GRAY,
            egui::Color32::DARK_GRAY,
            egui::Color32::BROWN,
        ];
        
        egui::Grid::new("color_palette")
            .num_columns(4)
            .spacing([2.0, 2.0])
            .show(ui, |ui| {
                for (i, &color) in colors.iter().enumerate() {
                    let size = egui::Vec2::new(20.0, 20.0);
                    let response = ui.allocate_response(size, egui::Sense::click());
                    
                    ui.painter().rect_filled(response.rect, 2.0, color);
                    ui.painter().rect_stroke(response.rect, 2.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
                    
                    if response.clicked() {
                        if ui.input(|i| i.modifiers.shift) {
                            self.secondary_color = color;
                        } else {
                            self.primary_color = color;
                        }
                    }
                    
                    if i % 4 == 3 {
                        ui.end_row();
                    }
                }
            });
        
        ui.separator();
        
        if ui.checkbox(&mut self.show_advanced, "Advanced").changed() {
            // Toggle advanced color picker
        }
        
        if self.show_advanced {
            self.show_advanced_picker(ui);
        }
    }
    
    fn show_advanced_picker(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("HSV", |ui| {
            // For now, we'll use a simpler RGB approach instead of HSV
            ui.label("HSV controls (coming soon)");
        });
        
        ui.collapsing("RGB", |ui| {
            let mut rgba = self.primary_color.to_array();
            
            ui.add(egui::Slider::new(&mut rgba[0], 0..=255).text("Red"));
            ui.add(egui::Slider::new(&mut rgba[1], 0..=255).text("Green"));
            ui.add(egui::Slider::new(&mut rgba[2], 0..=255).text("Blue"));
            ui.add(egui::Slider::new(&mut rgba[3], 0..=255).text("Alpha"));
            
            self.primary_color = egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
        });
    }
}
