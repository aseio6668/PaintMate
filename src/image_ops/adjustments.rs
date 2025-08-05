use image::{DynamicImage, ImageBuffer, Rgba};

pub fn adjust_brightness(img: &DynamicImage, brightness: f32) -> DynamicImage {
    let mut rgba_img = img.to_rgba8();
    
    for pixel in rgba_img.pixels_mut() {
        let r = (pixel[0] as f32 + brightness * 255.0).clamp(0.0, 255.0) as u8;
        let g = (pixel[1] as f32 + brightness * 255.0).clamp(0.0, 255.0) as u8;
        let b = (pixel[2] as f32 + brightness * 255.0).clamp(0.0, 255.0) as u8;
        
        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
    }
    
    DynamicImage::ImageRgba8(rgba_img)
}

pub fn adjust_contrast(img: &DynamicImage, contrast: f32) -> DynamicImage {
    let mut rgba_img = img.to_rgba8();
    let factor = (259.0 * (contrast * 255.0 + 255.0)) / (255.0 * (259.0 - contrast * 255.0));
    
    for pixel in rgba_img.pixels_mut() {
        let r = (factor * (pixel[0] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
        let g = (factor * (pixel[1] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
        let b = (factor * (pixel[2] as f32 - 128.0) + 128.0).clamp(0.0, 255.0) as u8;
        
        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
    }
    
    DynamicImage::ImageRgba8(rgba_img)
}

pub fn adjust_hue_saturation(img: &DynamicImage, hue_shift: f32, saturation: f32) -> DynamicImage {
    let mut rgba_img = img.to_rgba8();
    
    for pixel in rgba_img.pixels_mut() {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
        
        // Convert RGB to HSV
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;
        
        let mut h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };
        
        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;
        
        // Apply adjustments
        h = (h + hue_shift) % 360.0;
        let s = (s * saturation).clamp(0.0, 1.0);
        
        // Convert back to RGB
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r_prime, g_prime, b_prime) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        pixel[0] = ((r_prime + m) * 255.0) as u8;
        pixel[1] = ((g_prime + m) * 255.0) as u8;
        pixel[2] = ((b_prime + m) * 255.0) as u8;
    }
    
    DynamicImage::ImageRgba8(rgba_img)
}
