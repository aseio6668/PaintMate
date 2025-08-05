use eframe::egui;
use crate::image_ops::ImageData;

#[derive(Default)]
pub struct LayerPanel;

impl LayerPanel {
    pub fn show(&mut self, ui: &mut egui::Ui, image_data: &mut Option<ImageData>) {
        if let Some(ref mut img_data) = image_data {
            ui.horizontal(|ui| {
                if ui.button("Add").clicked() {
                    img_data.add_layer(format!("Layer {}", img_data.layers.len() + 1));
                }
                
                if ui.button("Delete").clicked() && img_data.layers.len() > 1 {
                    img_data.remove_layer(img_data.active_layer);
                }
                
                if ui.button("Duplicate").clicked() {
                    let layer = img_data.get_active_layer().clone();
                    img_data.layers.push(layer);
                    img_data.active_layer = img_data.layers.len() - 1;
                    img_data.mark_dirty();
                }
            });
            
            ui.separator();
            
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    // Show layers in reverse order (top layer first)
                    let mut mark_dirty = false;
                    let mut new_active_layer = img_data.active_layer;
                    
                    for i in (0..img_data.layers.len()).rev() {
                        let is_active = i == img_data.active_layer;
                        
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                // Visibility toggle
                                let mut visible = img_data.layers[i].visible;
                                if ui.checkbox(&mut visible, "").changed() {
                                    mark_dirty = true;
                                }
                                img_data.layers[i].visible = visible;
                                
                                // Layer name (clickable to select)
                                let name_response = ui.selectable_label(is_active, &img_data.layers[i].name);
                                if name_response.clicked() {
                                    new_active_layer = i;
                                }
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(format!("{:.0}%", img_data.layers[i].opacity * 100.0));
                                });
                            });
                            
                            // Opacity slider
                            let mut opacity = img_data.layers[i].opacity;
                            if ui.add(
                                egui::Slider::new(&mut opacity, 0.0..=1.0)
                                    .show_value(false)
                            ).changed() {
                                mark_dirty = true;
                            }
                            img_data.layers[i].opacity = opacity;
                            
                            // Blend mode
                            let mut blend_mode = img_data.layers[i].blend_mode.clone();
                            egui::ComboBox::from_id_source(format!("blend_mode_{}", i))
                                .selected_text(format!("{:?}", blend_mode))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut blend_mode, crate::image_ops::image_data::BlendMode::Normal, "Normal");
                                    ui.selectable_value(&mut blend_mode, crate::image_ops::image_data::BlendMode::Multiply, "Multiply");
                                    ui.selectable_value(&mut blend_mode, crate::image_ops::image_data::BlendMode::Screen, "Screen");
                                    ui.selectable_value(&mut blend_mode, crate::image_ops::image_data::BlendMode::Overlay, "Overlay");
                                });
                            img_data.layers[i].blend_mode = blend_mode;
                        });
                        
                        ui.separator();
                    }
                    
                    if mark_dirty {
                        img_data.mark_dirty();
                    }
                    img_data.active_layer = new_active_layer;
                });
        } else {
            ui.label("No image loaded");
        }
    }
}
