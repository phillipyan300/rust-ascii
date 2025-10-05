use anyhow::Result;
use clap::Parser;
use image::ImageReader;
use rust_ascii::{image_to_ascii, select_filter, select_ramp, validate_image_args};

#[derive(Parser)]
#[command(name = "asciirun")]
#[command(about = "Convert images to ASCII art")]
#[command(version)]
struct Args {
    /// Path to the input image file
    input: String,
    
    /// Number of columns in the output (default: 120)
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
    
    /// Output file path (default: stdout)
    #[arg(long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate arguments
    validate_image_args(args.cols, args.cell_aspect)?;

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
    
    // Output result
    match args.output {
        Some(path) => {
            std::fs::write(&path, ascii)
                .map_err(|e| anyhow::anyhow!("failed to write to {}: {e}", path))?;
            println!("ASCII art saved to: {}", path);
        }
        None => {
            print!("{ascii}");
        }
    }
    Ok(())
}

