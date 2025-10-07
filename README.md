# Rust ASCII Converter

A fast, multi-purpose command-line tool written in Rust that converts images and videos into grayscale ASCII art. Features both single-image conversion and parallel video-to-ASCII pipeline processing. The tool preserves the original aspect ratio while compensating for terminal cell shape, and uses configurable glyph ramps to map luminance to characters.

## Features

### Image Conversion
- **Fast Performance**: Converts images in milliseconds with optimized memory usage
- **Aspect Ratio Preservation**: Automatically calculates rows based on terminal cell aspect ratio
- **Multiple Resampling Filters**: Choose between nearest, triangle (bilinear), and Lanczos3
- **Configurable Glyph Ramps**: Use built-in presets or custom character sets
- **Format Support**: PNG, JPEG, GIF (first frame), and WebP images
- **Clean Output**: Pure ASCII output that works in any terminal or when redirected to files

### Video-to-ASCII Pipeline
- **Parallel Processing**: Multi-threaded frame conversion using all CPU cores
- **Terminal Animation**: Smooth ASCII movie playback with configurable FPS
- **One-Command Demo**: Complete pipeline from video to ASCII animation
- **FFmpeg Integration**: Automatic frame extraction with optimized settings
- **Performance Optimized**: Release builds for maximum conversion speed

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- FFmpeg (for video processing) - install via package manager or [ffmpeg.org](https://ffmpeg.org/)

### Build from Source

```bash
git clone <repository-url>
cd rust-ascii
cargo build --release
```

The binaries will be available at `target/release/`:
- `rust-ascii` - Single image converter
- `ascii-batch` - Parallel video frame converter  
- `ascii-play` - ASCII animation player

## Usage

### Image Conversion

#### Basic Usage

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

#### Command Line Options

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

#### Examples

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

### Video-to-ASCII Pipeline

#### Quick Start (One Command)

```bash
# Convert rickroll.mp4 to ASCII animation (30 FPS, 160 columns)
./scripts/run_demo.sh rickroll.mp4

# Custom settings: video, fps, columns, aspect ratio
./scripts/run_demo.sh rickroll.mp4 30 160 2.0
```

#### Manual Pipeline

```bash
# 1. Extract frames from video using FFmpeg
ffmpeg -i rickroll.mp4 -vf "fps=30,scale=160:-1:flags=area" frames/%06d.png

# 2. Convert frames to ASCII in parallel
cargo run --release --bin ascii-batch -- frames out_txt 160 2.0 triangle basic

# 3. Play the ASCII animation
cargo run --release --bin ascii-play -- out_txt 30
```

#### Pipeline Tools

**ascii-batch** - Parallel frame converter:
```bash
cargo run --release --bin ascii-batch -- [frames_dir] [out_dir] [cols] [cell_aspect] [resizer] [ramp]
```

**ascii-play** - ASCII animation player:
```bash
cargo run --release --bin ascii-play -- [txt_dir] [fps]
```

#### Video Processing Examples

```bash
# High quality conversion (more detail, slower)
./scripts/run_demo.sh rickroll.mp4 30 200 2.0

# Fast conversion (less detail, faster)
./scripts/run_demo.sh rickroll.mp4 15 120 2.0

# Custom FFmpeg extraction with different settings
ffmpeg -i rickroll.mp4 -vf "fps=24,scale=180:-1:flags=lanczos" frames/%06d.png
cargo run --release --bin ascii-batch -- frames out_txt 180 2.0 lanczos3 classic
cargo run --release --bin ascii-play -- out_txt 24
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

### Image Conversion
- **Typical Performance**: < 100ms for 1920×1080 images at 120 columns
- **Memory Efficient**: Pre-allocates output buffer, minimal temporary allocations
- **Optimized**: Single-pass conversion with tight loops

### Video Processing
- **Parallel Processing**: Near-linear speedup with CPU cores (e.g., 4x faster on 4-core system)
- **Frame Conversion**: ~50-200ms per frame depending on resolution and columns
- **Memory Efficient**: Processes frames independently, minimal memory overhead
- **Optimized Pipeline**: Release builds recommended for video processing

## Future Enhancements (v2+)

- **Color Support**: ANSI truecolor output
- **Advanced Features**: Dithering, Unicode braille mode, emoji mode
- **Streaming Support**: Real-time video processing without frame extraction
- **Audio Integration**: Synchronized audio playback with ASCII animations
- **Interactive Controls**: Pause, rewind, speed control during playback

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Technical Notes

- Built with Rust 2021 edition
- Uses the `image` crate for image processing
- CLI powered by `clap` with derive macros
- Error handling with `anyhow`
- Parallel processing with `rayon` for video pipeline
- Terminal control with `crossterm` for animation playback
- FFmpeg integration for video frame extraction
