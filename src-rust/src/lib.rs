mod arrs;
mod block;
mod consts;
mod cursor;
mod gf;
mod img;
pub mod target;
pub mod qr;

use image::{DynamicImage, RgbaImage};
use wasm_bindgen::prelude::*;

use crate::consts::{Version, side_len_of_version};

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
}

// add scale options eventually
#[wasm_bindgen]
pub fn prepare(
    version: u8,
    imagedata: Vec<u8>,
    brightness_threshold: u8,
    random: bool
) -> Vec<u8>{
    let version = Version::new(version).unwrap();
    let side_len = side_len_of_version(version);

    let rgbaimage = RgbaImage::from_raw(side_len, side_len, imagedata).expect("Could not convert imageData to imagebuffer");

    let output_image = qr::preview(DynamicImage::ImageRgba8(rgbaimage), version, brightness_threshold, random);
    // let output_image = target::preview(DynamicImage::ImageRgb8(rgbaimage), version, brightness_threshold);
    

    output_image.into_raw()
}

#[wasm_bindgen]
pub async fn generate(
    version: u8,
    url: String,
    imagedata: Vec<u8>,
    brightness_threshold: u8,
    random: bool,
    debug: bool,
    padding: u8,
    scale: u8,
) -> Vec<u8> {
    let version = Version::new(version).unwrap();
    let side_len = side_len_of_version(version);

    let rgbaimage = RgbaImage::from_raw(side_len, side_len, imagedata).expect("Could not convert imageData to imagebuffer");

    let output_image = qr::build(version, url, scale as u32, DynamicImage::ImageRgba8(rgbaimage), brightness_threshold, random, debug, padding).expect("Could not create qr code") ;

    output_image.into_raw()
}