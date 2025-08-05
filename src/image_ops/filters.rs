use image::DynamicImage;

pub fn blur(img: &DynamicImage, radius: f32) -> DynamicImage {
    img.blur(radius)
}

pub fn sharpen(img: &DynamicImage) -> DynamicImage {
    // Basic unsharp mask
    let blurred = img.blur(1.0);
    // TODO: Implement proper unsharp mask
    img.clone()
}

pub fn edge_detect(img: &DynamicImage) -> DynamicImage {
    // TODO: Implement edge detection filter
    img.clone()
}

pub fn emboss(img: &DynamicImage) -> DynamicImage {
    // TODO: Implement emboss filter
    img.clone()
}
