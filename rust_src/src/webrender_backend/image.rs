use image::ImageBuffer;
use webrender::api::ImageKey;

pub struct WrImage {
    // pub buffer: Vec<u8>,
    pub image_buffer: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

pub struct WrPixmap {
    pub image_key: ImageKey,
}
