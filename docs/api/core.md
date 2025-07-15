# Core Library API

Documentation for the Rust library API, enabling programmatic use of pixel art conversion functionality.

## Overview

The `pixel_art_rust` crate provides a high-level API for converting images to pixel art using various algorithms. The library is designed for performance and flexibility, supporting both fixed grid and adaptive approaches.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pixel_art_rust = "0.1.0"
```

## Basic Usage

```rust
use pixel_art_rust::{PixelArtConverter, Algorithm, GridConfig};
use image::open;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load input image
    let input_image = open("input.jpg")?;

    // Create converter with fixed grid
    let config = GridConfig::Fixed { width: 32, height: 32 };
    let converter = PixelArtConverter::new(config, Algorithm::Average);

    // Convert to pixel art
    let pixel_art = converter.convert(&input_image)?;

    // Save result
    pixel_art.save("output.png")?;

    Ok(())
}
```

## Core Types

### PixelArtConverter

The main converter struct that orchestrates the pixel art conversion process.

```rust
pub struct PixelArtConverter {
    grid_config: GridConfig,
    algorithm: Algorithm,
    color_config: Option<ColorConfig>,
}
```

#### Methods

**`new(grid_config: GridConfig, algorithm: Algorithm) -> Self`**

Creates a new converter with the specified configuration.

```rust
let converter = PixelArtConverter::new(
    GridConfig::Fixed { width: 64, height: 48 },
    Algorithm::KMeans
);
```

**`with_colors(mut self, color_config: ColorConfig) -> Self`**

Configures color quantization (required for MedianCut and KMeans algorithms).

```rust
let converter = PixelArtConverter::new(grid_config, Algorithm::KMeans)
    .with_colors(ColorConfig { count: 16 });
```

**`convert(&self, image: &DynamicImage) -> Result<DynamicImage, ConversionError>`**

Converts the input image to pixel art.

```rust
let result = converter.convert(&input_image)?;
```

**`convert_with_progress<F>(&self, image: &DynamicImage, callback: F) -> Result<DynamicImage, ConversionError>`**

Converts with progress callbacks.

```rust
let result = converter.convert_with_progress(&input_image, |progress| {
    println!("Progress: {:.1}%", progress * 100.0);
})?;
```

### GridConfig

Configuration for pixel grid layout.

```rust
pub enum GridConfig {
    Fixed { width: u32, height: u32 },
    Adaptive {
        max_depth: u32,
        variance_threshold: f64
    },
}
```

#### Variants

**`Fixed`**

- Creates uniform pixel grid
- `width`: Number of horizontal divisions
- `height`: Number of vertical divisions

**`Adaptive`**

- Uses quadtree for variable pixel sizes
- `max_depth`: Maximum subdivision depth (1-20)
- `variance_threshold`: Color variance threshold for splitting (0.0-100.0)

### Algorithm

Color extraction algorithms.

```rust
pub enum Algorithm {
    Average,
    MedianCut,
    KMeans,
}
```

#### Variants

**`Average`**

- Fast arithmetic mean calculation
- No additional configuration required
- Best for uniform images

**`MedianCut`**

- Recursive color space division
- Requires `ColorConfig`
- Balanced quality/performance

**`KMeans`**

- Iterative clustering algorithm
- Requires `ColorConfig`
- Highest quality results

### ColorConfig

Configuration for color quantization algorithms.

```rust
pub struct ColorConfig {
    pub count: u32,
}
```

**Fields:**

- `count`: Number of colors for quantization (2-256)

### ConversionError

Error type for conversion operations.

```rust
pub enum ConversionError {
    InvalidInput(String),
    ProcessingError(String),
    OutOfMemory,
    UnsupportedFormat,
}
```

## Advanced Usage

### Custom Progress Tracking

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

let progress = Arc::new(AtomicU32::new(0));
let progress_clone = progress.clone();

let result = converter.convert_with_progress(&image, move |p| {
    progress_clone.store((p * 100.0) as u32, Ordering::Relaxed);
})?;

println!("Final progress: {}%", progress.load(Ordering::Relaxed));
```

### Batch Processing

```rust
use std::path::Path;
use pixel_art_rust::{PixelArtConverter, Algorithm, GridConfig};

fn process_directory<P: AsRef<Path>>(
    input_dir: P,
    output_dir: P,
    converter: &PixelArtConverter,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = input_dir.as_ref();
    let output_path = output_dir.as_ref();

    std::fs::create_dir_all(output_path)?;

    for entry in std::fs::read_dir(input_path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if matches!(ext.to_str(), Some("jpg") | Some("png") | Some("jpeg")) {
                let input_image = image::open(&path)?;
                let pixel_art = converter.convert(&input_image)?;

                let output_file = output_path.join(
                    path.file_stem().unwrap()
                ).with_extension("png");

                pixel_art.save(output_file)?;
            }
        }
    }

    Ok(())
}
```

### Memory-Efficient Processing

```rust
use pixel_art_rust::{PixelArtConverter, Algorithm, GridConfig};

fn process_large_image(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Use adaptive mode for memory efficiency
    let config = GridConfig::Adaptive {
        max_depth: 8,
        variance_threshold: 40.0,
    };

    let converter = PixelArtConverter::new(config, Algorithm::Average);

    // Load and process
    let image = image::open(path)?;
    let result = converter.convert(&image)?;
    result.save("output.png")?;

    Ok(())
}
```

### Algorithm Comparison

```rust
use pixel_art_rust::{PixelArtConverter, Algorithm, GridConfig, ColorConfig};

fn compare_algorithms(input: &image::DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
    let grid = GridConfig::Fixed { width: 48, height: 48 };
    let colors = ColorConfig { count: 16 };

    // Average algorithm
    let avg_converter = PixelArtConverter::new(grid, Algorithm::Average);
    let avg_result = avg_converter.convert(input)?;
    avg_result.save("output_average.png")?;

    // Median Cut algorithm
    let median_converter = PixelArtConverter::new(grid, Algorithm::MedianCut)
        .with_colors(colors);
    let median_result = median_converter.convert(input)?;
    median_result.save("output_median.png")?;

    // K-Means algorithm
    let kmeans_converter = PixelArtConverter::new(grid, Algorithm::KMeans)
        .with_colors(colors);
    let kmeans_result = kmeans_converter.convert(input)?;
    kmeans_result.save("output_kmeans.png")?;

    Ok(())
}
```

## Performance Considerations

### Memory Usage

```rust
// Memory-efficient for large images
let config = GridConfig::Adaptive {
    max_depth: 6,  // Lower depth = less memory
    variance_threshold: 50.0,
};

// Memory-intensive but detailed
let config = GridConfig::Fixed {
    width: 256,    // High resolution = more memory
    height: 192,
};
```

### Threading

The library uses Rayon for parallel processing by default. Configure thread pool:

```rust
use rayon::ThreadPoolBuilder;

// Configure thread pool before first use
ThreadPoolBuilder::new()
    .num_threads(4)
    .build_global()
    .expect("Failed to configure thread pool");

// Now use converter normally
let converter = PixelArtConverter::new(config, algorithm);
```

### Algorithm Performance

| Algorithm | Speed  | Memory | Quality  | Best For                |
| --------- | ------ | ------ | -------- | ----------------------- |
| Average   | ⚡⚡⚡ | 💾     | ⭐⭐     | Large batches, previews |
| MedianCut | ⚡⚡   | 💾💾   | ⭐⭐⭐   | Balanced processing     |
| KMeans    | ⚡     | 💾💾💾 | ⭐⭐⭐⭐ | High-quality output     |

## Error Handling

### Comprehensive Error Handling

```rust
use pixel_art_rust::{PixelArtConverter, ConversionError};

fn safe_conversion(
    converter: &PixelArtConverter,
    image: &image::DynamicImage,
) -> Result<image::DynamicImage, String> {
    match converter.convert(image) {
        Ok(result) => Ok(result),
        Err(ConversionError::InvalidInput(msg)) => {
            Err(format!("Invalid input: {}", msg))
        },
        Err(ConversionError::ProcessingError(msg)) => {
            Err(format!("Processing failed: {}", msg))
        },
        Err(ConversionError::OutOfMemory) => {
            Err("Insufficient memory for processing".to_string())
        },
        Err(ConversionError::UnsupportedFormat) => {
            Err("Image format not supported".to_string())
        },
    }
}
```

## Feature Flags

Control compilation features in `Cargo.toml`:

```toml
[dependencies]
pixel_art_rust = { version = "0.1.0", features = ["parallel", "progress"] }
```

Available features:

- `parallel` (default): Enable parallel processing with Rayon
- `progress` (default): Enable progress callback support
- `simd`: Enable SIMD optimizations (requires nightly Rust)

## Integration Examples

### Web Assembly

```rust
use wasm_bindgen::prelude::*;
use pixel_art_rust::{PixelArtConverter, Algorithm, GridConfig};

#[wasm_bindgen]
pub fn convert_to_pixel_art(
    image_data: &[u8],
    width: u32,
    height: u32,
    grid_width: u32,
    grid_height: u32,
) -> Vec<u8> {
    let image = image::RgbaImage::from_raw(width, height, image_data.to_vec())
        .expect("Invalid image data");

    let converter = PixelArtConverter::new(
        GridConfig::Fixed {
            width: grid_width,
            height: grid_height
        },
        Algorithm::Average
    );

    let result = converter.convert(&image::DynamicImage::ImageRgba8(image))
        .expect("Conversion failed");

    result.to_rgba8().into_raw()
}
```

### Command Line Tool Integration

```rust
use clap::Parser;
use pixel_art_rust::{PixelArtConverter, Algorithm, GridConfig, ColorConfig};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, default_value = "32")]
    width: u32,

    #[arg(short = 'H', long, default_value = "32")]
    height: u32,

    #[arg(short, long, default_value = "average")]
    algorithm: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let algorithm = match args.algorithm.as_str() {
        "average" => Algorithm::Average,
        "median-cut" => Algorithm::MedianCut,
        "kmeans" => Algorithm::KMeans,
        _ => panic!("Invalid algorithm"),
    };

    let config = GridConfig::Fixed {
        width: args.width,
        height: args.height,
    };

    let mut converter = PixelArtConverter::new(config, algorithm);

    if matches!(algorithm, Algorithm::MedianCut | Algorithm::KMeans) {
        converter = converter.with_colors(ColorConfig { count: 16 });
    }

    let image = image::open(&args.input)?;
    let result = converter.convert(&image)?;
    result.save(&args.output)?;

    Ok(())
}
```

## See Also

- [CLI Reference](/api/cli) - Command-line interface documentation
- [Algorithm Details](/algorithms/overview) - Technical algorithm explanations
- [Usage Examples](/guide/examples) - Practical usage examples
