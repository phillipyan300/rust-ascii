//! Image processing module for converting images to ASCII art

use anyhow::Result;
use image::{DynamicImage, imageops::FilterType, GenericImageView};

/// Select the appropriate image filter based on the resizer name
pub fn select_filter(name: &str) -> Result<Option<FilterType>> {
    Ok(match name {
        "nearest" => Some(FilterType::Nearest),
        "triangle" => Some(FilterType::Triangle),
        "lanczos3" => Some(FilterType::Lanczos3),
        "pixel" => None, // Special case for pixel-by-pixel sampling
        "1to1" => None, // Special case for 1:1 pixel mapping
        other => anyhow::bail!("unknown resizer: {other}. Available options: nearest, triangle, lanczos3, pixel, 1to1"),
    })
}

/// Select and validate the glyph ramp for ASCII conversion
pub fn select_ramp(spec: &str) -> Result<Vec<u8>> {
    let s = match spec {
        "basic" => " .:-=+*#%@",
        "classic" => " .'`^\",:;Il!i<>*+_-?][}{)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$",
        _ => spec, // allow custom string directly
    };
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        anyhow::bail!("ramp cannot be empty");
    }
    Ok(bytes.to_vec())
}

/// Convert an image to ASCII art using the specified parameters
pub fn image_to_ascii(
    img: &DynamicImage,
    cols: u32,
    cell_aspect: f32,
    filter: Option<FilterType>,
    resizer_name: &str,
    ramp: Vec<u8>,
) -> Result<String> {
    let (w, h) = img.dimensions();
    let ramp_len = ramp.len();
    
    // Preallocate output string for performance
    let mut out = String::new();
    
    match resizer_name {
        "1to1" => {
            // 1:1 pixel mapping - each original pixel becomes one ASCII character
            // Ignore cols parameter, use original image dimensions
            out.reserve(((w + 1) * h) as usize);
            let gray = img.to_luma8();
            
            for y in 0..h {
                for x in 0..w {
                    let pixel = gray.get_pixel(x, y)[0];
                    let idx = ((pixel as usize * (ramp_len - 1)) / 255).min(ramp_len - 1);
                    out.push(ramp[idx] as char);
                }
                out.push('\n');
            }
        }
        _ => {
            // Calculate scale factor and output rows for other modes
            let scale = cols as f32 / w as f32;
            let rows = ((h as f32 * scale) / cell_aspect).max(1.0).round() as u32;
            
            // Preallocate output string for performance (+1 for newline per row)
            out.reserve(((cols + 1) * rows) as usize);
            
            match filter {
                Some(filter_type) => {
                    // Traditional approach: resize first, then convert
                    let gray = img.resize_exact(cols, rows, filter_type).to_luma8();
                    
                    for y in 0..rows {
                        for x in 0..cols {
                            let pixel = gray.get_pixel(x, y)[0];
                            let idx = ((pixel as usize * (ramp_len - 1)) / 255).min(ramp_len - 1);
                            out.push(ramp[idx] as char);
                        }
                        out.push('\n');
                    }
                }
                None => {
                    // Pixel-by-pixel approach: sample original image directly
                    let gray = img.to_luma8();
                    
                    for y in 0..rows {
                        for x in 0..cols {
                            // Calculate the position in the original image
                            let orig_x = ((x as f32 + 0.5) / scale) as u32;
                            let orig_y = ((y as f32 + 0.5) / scale * cell_aspect) as u32;
                            
                            // Clamp to image bounds
                            let orig_x = orig_x.min(w - 1);
                            let orig_y = orig_y.min(h - 1);
                            
                            let pixel = gray.get_pixel(orig_x, orig_y)[0];
                            let idx = ((pixel as usize * (ramp_len - 1)) / 255).min(ramp_len - 1);
                            out.push(ramp[idx] as char);
                        }
                        out.push('\n');
                    }
                }
            }
        }
    }
    
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Luma};

    #[test]
    fn test_select_filter() {
        assert!(select_filter("triangle").unwrap().is_some());
        assert!(select_filter("pixel").unwrap().is_none());
        assert!(select_filter("1to1").unwrap().is_none());
        assert!(select_filter("invalid").is_err());
    }

    #[test]
    fn test_select_ramp() {
        let basic = select_ramp("basic").unwrap();
        assert_eq!(basic, b" .:-=+*#%@");
        
        let custom = select_ramp("abc").unwrap();
        assert_eq!(custom, b"abc");
        
        assert!(select_ramp("").is_err());
    }

    #[test]
    fn test_image_to_ascii_1to1() {
        // Create a simple 2x2 test image
        let img = ImageBuffer::from_fn(2, 2, |x, y| {
            Luma([if (x + y) % 2 == 0 { 0 } else { 255 }])
        });
        let dynamic_img = DynamicImage::ImageLuma8(img);
        
        let ramp = select_ramp("basic").unwrap();
        let result = image_to_ascii(&dynamic_img, 10, 2.0, None, "1to1", ramp).unwrap();
        
        // Should have 2 lines (2 rows)
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 2);
    }
}
