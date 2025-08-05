use std::path::Path;
use anyhow::Result;
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use eframe::egui;

#[derive(Clone)]
pub struct ImageData {
    pub layers: Vec<Layer>,
    pub active_layer: usize,
    width: u32,
    height: u32,
    texture_handle: Option<egui::TextureHandle>,
    needs_update: bool,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub data: RgbaImage,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Darken,
    Lighten,
    Difference,
    Exclusion,
}

impl ImageData {
    pub fn new(width: u32, height: u32) -> Self {
        let mut layers = Vec::new();
        layers.push(Layer::new("Background".to_string(), width, height));
        
        Self {
            layers,
            active_layer: 0,
            width,
            height,
            texture_handle: None,
            needs_update: true,
        }
    }
    
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let img = image::open(path)?;
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();
        
        let mut layers = Vec::new();
        layers.push(Layer {
            name: "Background".to_string(),
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            data: rgba_img,
        });
        
        Ok(Self {
            layers,
            active_layer: 0,
            width,
            height,
            texture_handle: None,
            needs_update: true,
        })
    }
    
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let flattened = self.flatten();
        flattened.save(path)?;
        Ok(())
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn add_layer(&mut self, name: String) {
        let layer = Layer::new(name, self.width, self.height);
        self.layers.push(layer);
        self.active_layer = self.layers.len() - 1;
        self.needs_update = true;
    }
    
    pub fn remove_layer(&mut self, index: usize) {
        if self.layers.len() > 1 && index < self.layers.len() {
            self.layers.remove(index);
            if self.active_layer >= self.layers.len() {
                self.active_layer = self.layers.len() - 1;
            }
            self.needs_update = true;
        }
    }
    
    pub fn get_active_layer_mut(&mut self) -> &mut Layer {
        &mut self.layers[self.active_layer]
    }
    
    pub fn get_active_layer(&self) -> &Layer {
        &self.layers[self.active_layer]
    }
    
    pub fn flatten(&self) -> RgbaImage {
        let mut result = ImageBuffer::new(self.width, self.height);
        
        // Fill with transparent pixels
        for pixel in result.pixels_mut() {
            *pixel = Rgba([255, 255, 255, 0]);
        }
        
        // Blend layers from bottom to top
        for layer in &self.layers {
            if layer.visible {
                self.blend_layer(&mut result, layer);
            }
        }
        
        result
    }
    
    fn blend_layer(&self, base: &mut RgbaImage, layer: &Layer) {
        for (x, y, base_pixel) in base.enumerate_pixels_mut() {
            if x < layer.data.width() && y < layer.data.height() {
                let layer_pixel = layer.data.get_pixel(x, y);
                let blended = self.blend_pixels(*base_pixel, *layer_pixel, &layer.blend_mode, layer.opacity);
                *base_pixel = blended;
            }
        }
    }
    
    fn blend_pixels(&self, base: Rgba<u8>, overlay: Rgba<u8>, blend_mode: &BlendMode, opacity: f32) -> Rgba<u8> {
        let base_alpha = base[3] as f32 / 255.0;
        let overlay_alpha = (overlay[3] as f32 / 255.0) * opacity;
        
        if overlay_alpha == 0.0 {
            return base;
        }
        
        let result_alpha = overlay_alpha + base_alpha * (1.0 - overlay_alpha);
        
        if result_alpha == 0.0 {
            return Rgba([0, 0, 0, 0]);
        }
        
        let mut result = [0u8; 4];
        
        for i in 0..3 {
            let base_c = base[i] as f32 / 255.0;
            let overlay_c = overlay[i] as f32 / 255.0;
            
            let blended_c = match blend_mode {
                BlendMode::Normal => overlay_c,
                BlendMode::Multiply => base_c * overlay_c,
                BlendMode::Screen => 1.0 - (1.0 - base_c) * (1.0 - overlay_c),
                BlendMode::Overlay => {
                    if base_c < 0.5 {
                        2.0 * base_c * overlay_c
                    } else {
                        1.0 - 2.0 * (1.0 - base_c) * (1.0 - overlay_c)
                    }
                }
                _ => overlay_c, // Simplified for now
            };
            
            let final_c = (blended_c * overlay_alpha + base_c * base_alpha * (1.0 - overlay_alpha)) / result_alpha;
            result[i] = (final_c * 255.0).round() as u8;
        }
        
        result[3] = (result_alpha * 255.0).round() as u8;
        Rgba(result)
    }
    
    pub fn get_texture(&mut self, ctx: &egui::Context) -> egui::TextureHandle {
        if self.texture_handle.is_none() || self.needs_update {
            let flattened = self.flatten();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [self.width as usize, self.height as usize],
                &flattened.as_raw(),
            );
            
            self.texture_handle = Some(ctx.load_texture(
                "canvas",
                color_image,
                egui::TextureOptions::NEAREST,
            ));
            self.needs_update = false;
        }
        
        self.texture_handle.as_ref().unwrap().clone()
    }
    
    pub fn mark_dirty(&mut self) {
        self.needs_update = true;
    }
    
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Rgba<u8>) {
        if x < self.width && y < self.height {
            let layer = self.get_active_layer_mut();
            layer.data.put_pixel(x, y, color);
            self.needs_update = true;
        }
    }
    
    pub fn draw_circle(&mut self, center_x: u32, center_y: u32, radius: f32, color: Rgba<u8>) {
        let width = self.width;
        let height = self.height;
        let layer = self.get_active_layer_mut();
        
        for y in 0..height {
            for x in 0..width {
                let dx = x as f32 - center_x as f32;
                let dy = y as f32 - center_y as f32;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    layer.data.put_pixel(x, y, color);
                }
            }
        }
        
        self.needs_update = true;
    }
}

impl Layer {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        let data = ImageBuffer::from_fn(width, height, |_, _| Rgba([255, 255, 255, 0]));
        
        Self {
            name,
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            data,
        }
    }
}
