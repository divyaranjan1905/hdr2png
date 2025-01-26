use clap::Parser;
use image::ImageResult;
use std::path::Path;
mod cli;
mod encode;

fn main() -> ImageResult<()> {
    let cli_args = cli::Cli::parse();
    let hdr_path = Path::new(&cli_args.input);
    let output = cli_args
        .output
        .unwrap_or_else(|| cli_args.input.trim_end_matches(".hdr").to_owned() + ".png");
    let png_path = Path::new(&output);

    match encode::load_hdr(hdr_path) {
        Ok((hdr_rgb_float, dim_vec)) => {
            let width = dim_vec[0];
            let height = dim_vec[1];
            let hdr_rgbm_float: Vec<_> = hdr_rgb_float
                .iter()
                .cloned()
                .map(|x| encode::encode_rgbe_float_to_rgbm(x))
                .collect();

            encode::save_rgbm_png_to_file(png_path, width, height, hdr_rgbm_float)
        }

        Err(e) => {
            println!("Error loading the HDR file: {}", e);
            Err(e)
        }
    }
}
