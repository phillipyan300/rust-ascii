# Rust ASCII Image Converter

A fast, single-threaded command-line tool written in Rust that converts raster images into grayscale ASCII art. The tool preserves the original image aspect ratio while compensating for terminal cell shape, and uses configurable glyph ramps to map luminance to characters.

## Features

- **Fast Performance**: Converts images in milliseconds with optimized memory usage
- **Aspect Ratio Preservation**: Automatically calculates rows based on terminal cell aspect ratio
- **Multiple Resampling Filters**: Choose between nearest, triangle (bilinear), and Lanczos3
- **Configurable Glyph Ramps**: Use built-in presets or custom character sets
- **Format Support**: PNG, JPEG, GIF (first frame), and WebP images
- **Clean Output**: Pure ASCII output that works in any terminal or when redirected to files

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Build from Source

```bash
git clone <repository-url>
cd rust-ascii
cargo build --release
```

The binary will be available at `target/release/rust-ascii`.

## Usage

### Basic Usage

```bash
# Convert an image with default settings (120 columns)
rust-ascii image.jpg

# Specify output width
rust-ascii image.png --cols 80

# Use different resampling filter
rust-ascii image.jpg --resizer lanczos3

# Adjust for your terminal's character cell aspect ratio
rust-ascii image.png --cell-aspect 1.8
```

### Command Line Options

```
rust-ascii <INPUT> [OPTIONS]

Arguments:
  <INPUT>  Path to the input image file

Options:
      --cols <COLS>                Number of columns in the output [default: 120]
      --cell-aspect <CELL_ASPECT>  Terminal cell aspect ratio (height/width) [default: 2.0]
      --resizer <RESIZER>          Resampling filter (nearest, triangle, lanczos3) [default: triangle]
      --ramp <RAMP>                Glyph ramp for mapping luminance to characters [default: basic]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Examples

```bash
# High detail output with Lanczos3 resampling
rust-ascii photo.jpg --cols 160 --resizer lanczos3

# Compact output with custom character ramp
rust-ascii logo.png --cols 60 --ramp " .:-=+*#%@"

# Adjust for wide terminal characters
rust-ascii landscape.jpg --cell-aspect 1.5 --cols 100

# Use the classic detailed ramp
rust-ascii portrait.jpg --ramp classic --cols 120
```

## Configuration Options

### Resampling Filters

- **`nearest`**: Fastest, picks one source pixel per destination pixel (can appear jagged)
- **`triangle`**: Good balance of speed and quality, blends neighboring pixels (default)
- **`lanczos3`**: Highest quality, produces sharper downscales with more computation

### Glyph Ramps

- **`basic`**: `" .:-=+*#%@"` - Simple 10-character ramp (default)
- **`classic`**: `" .'`^\",:;Il!i<>*+_-?][}{)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"` - Detailed 70-character ramp
- **Custom**: Provide your own string of characters ordered from light to dark

### Cell Aspect Ratio

The `--cell-aspect` parameter compensates for terminal character cell geometry:
- **2.0** (default): Standard for most monospace fonts
- **1.8-2.2**: Typical range for different fonts and terminals
- **1.5**: For wider character cells
- **2.5**: For taller character cells

## Algorithm Details

### Character Grid Sizing

The tool calculates output dimensions using:
- Scale factor: `s = cols / image_width`
- Output rows: `rows = round((image_height * s) / cell_aspect)`

This preserves the original image's aspect ratio when displayed in terminals where characters are taller than they are wide.

### Luminance Mapping

Each pixel's luminance is normalized to `[0, 1]` and mapped to a character:
```
index = round(luminance * (ramp_length - 1))
character = ramp[index]
```

## Performance

- **Typical Performance**: < 100ms for 1920×1080 images at 120 columns
- **Memory Efficient**: Pre-allocates output buffer, minimal temporary allocations
- **Optimized**: Single-pass conversion with tight loops

## Supported Image Formats

- PNG
- JPEG
- GIF (first frame only)
- WebP (where supported by the image crate)

## Error Handling

The tool provides clear error messages for:
- Invalid file paths or unsupported formats
- Invalid command-line arguments
- Image decoding failures
- Empty or invalid glyph ramps

## Examples

### Test Pattern Output

Here's what a test pattern looks like converted to ASCII:

```
  #@@@#.   +@@@%:   -@@@@-   .%@@@*    #@@@#.   +@@@%:   -@@@@-   :%@@@+   .#@@@#  
::#%%%#:...+%%%%-...=%%%%=...-%%%%*:..:#%%%#:...+%%%%-...=%%%%=...-%%%%+...:#%%%#::
%%*+++*++++*+++*++++**+++*++++*+++*++++*+++*++++*+++*++++++++++++++*+++*++++*+++*%%
@@*+++*++++*+++**+++*++++*++++++++*++++*+++*++++*+++***=--::::--::--==+*++++*+++*@@
```

## Future Enhancements (v2+)

- **Color Support**: ANSI truecolor output
- **Video Processing**: Frame-by-frame conversion
- **Parallel Processing**: Multi-threaded conversion
- **Advanced Features**: Dithering, Unicode braille mode, emoji mode

## License

This project is open source. See the license file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Technical Notes

- Built with Rust 2021 edition
- Uses the `image` crate for image processing
- CLI powered by `clap` with derive macros
- Error handling with `anyhow`
- Single-threaded design for simplicity and reliability
