/// Module for handling the CLI parsing
use clap::{Parser, ValueEnum};

// Enum for the format options available in the CLI
#[derive(ValueEnum, Clone)]
pub enum Format {
    Rgba,
    Rgbm,
}

/// Commandline tool to encode Radiance HDR images into RGBM/RGBA PNG
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input HDR file
    #[arg(
        long,
        value_name = "HDR_FILE",
        required = true,
        help = "Path to the input HDR file"
    )]
    pub input: String,

    /// Output PNG file
    #[arg(long, value_name = "PNG_FILE", help = "Path to the output PNG file")]
    pub output: Option<String>, // Option because this is an optional argument.

    #[arg(
        long,
        value_enum,
        value_name = "ENCODING_FORMAT",
        help = "The encoding format to use for PNG"
    )]
    pub format: Format,
}
