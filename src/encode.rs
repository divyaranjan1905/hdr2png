use crate::cli::{Cli, Format};
use clap::Parser;
use image::codecs::png::*;
use image::{ExtendedColorType, ImageEncoder, ImageError, ImageResult};
use std::fs::File;
use std::io::Write;
use std::path::Path;

const RANGE: f32 = 6.0;

pub fn load_hdr(hdr_path: &Path) -> Result<(Vec<Vec<f32>>, Vec<u32>), image::ImageError> {
    let hdr_file = Path::new(hdr_path);
    let hdr_rgbe8 = rgbe::load_radiance_file(hdr_file);
    let mut hdr_vec = Vec::new();
    let mut dim_vec = Vec::new();

    match hdr_rgbe8 {
        Ok((width, height, texels)) => {
            hdr_vec.extend(texels);
            dim_vec.push(width);
            dim_vec.push(height);
        }

        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let hdr_rgb_float: Vec<_> = hdr_vec.iter().map(|x| x.unpack().to_vec()).collect();
    Ok((hdr_rgb_float, dim_vec))
}

pub fn encode_rgbe_float_to_rgbm(rgbe_vec: Vec<f32>) -> Vec<f32> {
    let mut rgbm_float_vec: Vec<f32> = rgbe_vec.iter().map(|x| x / RANGE).collect();
    let rgb_max = rgbm_float_vec.iter().cloned().fold(f32::MIN, f32::max);
    let multi = (rgb_max * 255.0).ceil() / 255.0;

    rgbm_float_vec = rgbm_float_vec.iter().map(|x| x / multi).collect();
    rgbm_float_vec.push(multi);
    rgbm_float_vec
}

pub fn convert_rgbm_float_to_8_bit(rgbm_float_vec: Vec<f32>) -> Vec<u8> {
    let rgbm8_vec: Vec<u8> = rgbm_float_vec
        .iter()
        .map(|x| (x * 255.0).ceil() as u8)
        .collect();
    rgbm8_vec
}

pub fn decode_rgbm_to_rgba8(rgbm_float_vec: Vec<f32>) -> Vec<u8> {
    let multi = rgbm_float_vec[3];
    let mut rgbma_vec: Vec<u8> = rgbm_float_vec
        .iter()
        .map(|x| (x * RANGE * multi * 255.0) as u8)
        .collect();
    rgbma_vec[3] = 255;
    rgbma_vec
}

pub fn encode_rgbm_to_png<W: Write>(
    width: u32,
    height: u32,
    rgbm_data: Vec<Vec<f32>>,
    output_file: W,
) -> ImageResult<()> {
    let encoder =
        PngEncoder::new_with_quality(output_file, CompressionType::Best, FilterType::Adaptive);
    let rgba8_data: Vec<_> = rgbm_data
        .iter()
        .cloned()
        .map(|x| decode_rgbm_to_rgba8(x))
        .collect();

    let rgbm8_data: Vec<_> = rgbm_data
        .iter()
        .cloned()
        .map(|x| convert_rgbm_float_to_8_bit(x))
        .collect();
    // let concat_rgb_data: Vec<u8> = rgbm8_data.into_iter().flatten().collect();
    let mut concat_rgb_data: Vec<u8> = Vec::new();
    let rgba_concat: Vec<u8> = rgba8_data.into_iter().flatten().collect();
    let rgbm_concat: Vec<u8> = rgbm8_data.into_iter().flatten().collect();

    let cli_args = Cli::parse();
    match cli_args.format {
        Format::Rgba => concat_rgb_data.extend(rgba_concat),
        Format::Rgbm => concat_rgb_data.extend(rgbm_concat),
    }

    let concat_rgb_slice = concat_rgb_data.as_slice();

    encoder.write_image(concat_rgb_slice, width, height, ExtendedColorType::Rgba8)?;
    Ok(())
}

pub fn save_rgbm_png_to_file(
    png_path: &Path,
    width: u32,
    height: u32,
    rgbm_data: Vec<Vec<f32>>,
) -> ImageResult<()> {
    let output_png_file = File::create(png_path).map_err(ImageError::IoError)?;
    encode_rgbm_to_png(width, height, rgbm_data, output_png_file)
}
