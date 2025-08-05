pub mod image_data;
pub mod history;
pub mod adjustments;
pub mod filters;
pub mod clipboard_ops;

use std::path::Path;
use anyhow::Result;
use image::{DynamicImage, ImageBuffer, Rgba};

pub use image_data::ImageData;
pub use history::ImageHistory;
pub use adjustments::*;
pub use filters::*;
pub use clipboard_ops::*;

pub type RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub struct ImageOperations;

impl ImageOperations {
    pub fn resize(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
        img.resize(width, height, image::imageops::FilterType::Lanczos3)
    }
    
    pub fn crop(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
        img.crop_imm(x, y, width, height)
    }
    
    pub fn rotate_90(img: &DynamicImage) -> DynamicImage {
        img.rotate90()
    }
    
    pub fn rotate_180(img: &DynamicImage) -> DynamicImage {
        img.rotate180()
    }
    
    pub fn rotate_270(img: &DynamicImage) -> DynamicImage {
        img.rotate270()
    }
    
    pub fn flip_horizontal(img: &DynamicImage) -> DynamicImage {
        img.fliph()
    }
    
    pub fn flip_vertical(img: &DynamicImage) -> DynamicImage {
        img.flipv()
    }
}
