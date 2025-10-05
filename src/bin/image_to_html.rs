use anyhow::Result;
use clap::Parser;
use image::ImageReader;
use rust_ascii::{
    image_to_ascii, select_filter, select_ramp, ascii_to_html, HtmlConfig,
    validate_image_args, validate_hex_color, print_conversion_summary
};
use std::fs;

// Runs the full workflow, converting an image to ASCII art and then to HTML

#[derive(Parser)]
#[command(name = "image-to-html")]
#[command(about = "Convert image directly to HTML ASCII art visualization")]
#[command(version)]
struct Args {
    /// Path to the input image file
    input: String,
    
    /// Output HTML file path
    #[arg(long, default_value = "ascii_art.html")]
    output: String,
    
    /// Number of columns in the output (ignored for 1to1 mode)
    #[arg(long, default_value_t = 120)]
    cols: u32,
    
    /// Terminal cell aspect ratio (height/width, default: 2.0)
    #[arg(long, default_value_t = 2.0)]
    cell_aspect: f32,
    
    /// Resampling filter to use (nearest, triangle, lanczos3, pixel, 1to1)
    #[arg(long, default_value = "triangle")]
    resizer: String,
    
    /// Glyph ramp for mapping luminance to characters
    #[arg(long, default_value = "basic")]
    ramp: String,
    
    /// Font size in pixels
    #[arg(long, default_value_t = 1)]
    font_size: u32,
    
    /// Background color (hex, e.g., 000000 for black)
    #[arg(long, default_value = "000000")]
    background: String,
    
    /// Text color (hex, e.g., ffffff for white)
    #[arg(long, default_value = "ffffff")]
    text_color: String,
    
    /// Font family
    #[arg(long, default_value = "monospace")]
    font_family: String,
}


fn main() -> Result<()> {
    let args = Args::parse();

    // Validate arguments
    validate_image_args(args.cols, args.cell_aspect)?;
    validate_hex_color(&args.background)?;
    validate_hex_color(&args.text_color)?;

    println!("Converting {} to ASCII art...", args.input);

    // Load and decode image
    let img = ImageReader::open(&args.input)
        .map_err(|e| anyhow::anyhow!("failed to open {}: {e}", args.input))?
        .decode()
        .map_err(|e| anyhow::anyhow!("failed to decode {}: {e}", args.input))?;

    // Select filter and ramp
    let filter = select_filter(&args.resizer)?;
    let ramp = select_ramp(&args.ramp)?;
    
    // Convert to ASCII
    let ascii = image_to_ascii(&img, args.cols, args.cell_aspect, filter, &args.resizer, ramp)?;
    
    // Generate HTML
    let lines: Vec<&str> = ascii.lines().collect();
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let height = lines.len();
    
    let html_config = HtmlConfig {
        font_size: args.font_size,
        background_color: args.background,
        text_color: args.text_color,
        font_family: args.font_family,
    };
    
    let html = ascii_to_html(&ascii, html_config)?;
    
    // Save HTML file
    fs::write(&args.output, html)
        .map_err(|e| anyhow::anyhow!("failed to write HTML to {}: {e}", args.output))?;
    
    print_conversion_summary(
        &args.input,
        &args.output,
        (max_width, height),
        args.font_size,
        &args.resizer,
    );
    
    Ok(())
}
