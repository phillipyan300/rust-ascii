//! Shared CLI utilities and validation functions

use anyhow::{Result, bail};

/// Validate CLI arguments for image processing
pub fn validate_image_args(cols: u32, cell_aspect: f32) -> Result<()> {
    if cols == 0 {
        bail!("--cols must be positive");
    }
    if !(0.5..=5.0).contains(&cell_aspect) {
        bail!("--cell-aspect should be between 0.5 and 5.0");
    }
    Ok(())
}

/// Validate hex color format
pub fn validate_hex_color(hex: &str) -> Result<()> {
    if hex.len() != 6 {
        bail!("Color must be 6 hex digits (e.g., ffffff)");
    }
    u32::from_str_radix(hex, 16)
        .map_err(|_| anyhow::anyhow!("Invalid hex color: {}", hex))?;
    Ok(())
}

/// Print conversion summary
pub fn print_conversion_summary(
    input_path: &str,
    output_path: &str,
    dimensions: (usize, usize),
    font_size: u32,
    resizer: &str,
) {
    println!("✅ Conversion complete!");
    println!("📁 Input: {}", input_path);
    println!("📄 Output: {}", output_path);
    println!("📏 ASCII dimensions: {}x{} characters", dimensions.0, dimensions.1);
    println!("🔤 Font size: {}px", font_size);
    println!("🎨 Resizer: {}", resizer);
    println!("\n🌐 Open the HTML file in your browser to view the ASCII art!");
    println!("⌨️  Use Ctrl/Cmd + +/- to zoom in/out, Ctrl/Cmd + 0 to reset zoom.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_image_args() {
        assert!(validate_image_args(120, 2.0).is_ok());
        assert!(validate_image_args(0, 2.0).is_err());
        assert!(validate_image_args(120, 0.1).is_err());
        assert!(validate_image_args(120, 10.0).is_err());
    }

    #[test]
    fn test_validate_hex_color() {
        assert!(validate_hex_color("ffffff").is_ok());
        assert!(validate_hex_color("000000").is_ok());
        assert!(validate_hex_color("ff0000").is_ok());
        assert!(validate_hex_color("gggggg").is_err());
        assert!(validate_hex_color("ffff").is_err());
    }
}
