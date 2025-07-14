# CLI Reference

Complete command-line interface documentation for Pixel Art Rust.

## Synopsis

```bash
pixel-art-rust [OPTIONS] -i <INPUT> -o <OUTPUT>
```

## Required Arguments

### Input/Output

**`-i, --input <PATH>`**
- Path to the input image file
- Supported formats: JPG, JPEG, PNG, GIF, BMP, WebP, TGA, DDS
- Can be absolute or relative path
- Example: `-i photo.jpg` or `-i /path/to/image.png`

**`-o, --output <PATH>`**
- Path for the output pixel art image
- Recommended format: PNG (lossless compression)
- Will create directories if they don't exist
- Example: `-o pixel_art.png` or `-o output/result.png`

## Grid Configuration

### Fixed Grid Mode (Default)

**`-w, --width <WIDTH>`**
- Number of horizontal pixel divisions
- Must be positive integer
- Range: 1-1000 (practical limit depends on input size)
- Example: `-w 32` creates 32 columns

**`-h, --height <HEIGHT>`**
- Number of vertical pixel divisions  
- Must be positive integer
- Range: 1-1000 (practical limit depends on input size)
- Example: `-h 24` creates 24 rows

### Adaptive Mode

**`--adaptive`**
- Enable adaptive quadtree mode instead of fixed grid
- Dynamically adjusts pixel size based on image complexity
- Cannot be used with `-w` or `-h` options
- Better for images with varying detail levels

**`--max-depth <DEPTH>`**
- Maximum depth for quadtree subdivision
- Default: 10
- Range: 1-20
- Higher values allow more detail but increase processing time
- Only used with `--adaptive`

**`--variance-threshold <VALUE>`**
- Color variance threshold for quadtree splitting
- Default: 50.0
- Range: 0.0-100.0
- Lower values create more subdivisions (more detail)
- Only used with `--adaptive`

## Algorithm Selection

**`-a, --algorithm <ALGORITHM>`**
- Color quantization algorithm to use
- Default: `average`
- Available options:

### Algorithm Options

**`average`**
- Arithmetic mean of all pixels in each cell
- Fastest processing
- Good for uniform images and quick previews
- Memory efficient

**`median-cut`**
- Recursive color space division algorithm
- Balanced quality and performance
- Good color distribution
- Requires `--colors` parameter

**`kmeans`**
- K-means clustering for optimal color selection
- Highest quality results
- Slowest processing
- Requires `--colors` parameter

## Color Quantization

**`-c, --colors <COLORS>`**
- Number of colors for quantization algorithms
- Required for `median-cut` and `kmeans` algorithms
- Ignored for `average` algorithm
- Range: 2-256
- Default: 16 (when required)
- Lower values create more stylized results

## Help and Version

**`-h, --help`**
- Display help information and exit
- Shows all available options and usage examples

**`-V, --version`**
- Display version information and exit
- Shows current version and build information

## Usage Examples

### Basic Usage

**Simple conversion:**
```bash
pixel-art-rust -w 32 -h 32 -i photo.jpg -o pixel.png
```

**High resolution:**
```bash
pixel-art-rust -w 128 -h 96 -i landscape.jpg -o detailed.png
```

### Algorithm Examples

**Using median-cut:**
```bash
pixel-art-rust -w 64 -h 64 -i image.jpg -o output.png --algorithm median-cut --colors 16
```

**Using k-means for highest quality:**
```bash
pixel-art-rust -w 48 -h 48 -i portrait.jpg -o quality.png --algorithm kmeans --colors 32
```

**Adaptive quadtree mode:**
```bash
pixel-art-rust --adaptive -i complex.jpg -o adaptive.png --max-depth 8 --variance-threshold 25.0
```

### Creative Examples

**Retro 8-bit style:**
```bash
pixel-art-rust -w 32 -h 32 -i modern.jpg -o retro.png --algorithm median-cut --colors 4
```

**High detail with many colors:**
```bash
pixel-art-rust -w 100 -h 75 -i photo.jpg -o detailed.png --algorithm kmeans --colors 128
```

**Minimalist style:**
```bash
pixel-art-rust -w 16 -h 16 -i busy.jpg -o simple.png --algorithm median-cut --colors 3
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error (invalid arguments, file not found, etc.) |
| 2 | Input/output error (file permissions, corrupted image, etc.) |
| 3 | Processing error (out of memory, algorithm failure, etc.) |

## Error Messages

### Common Errors

**Input file not found:**
```
Error: Input file 'image.jpg' not found
```
Solution: Check file path and permissions

**Invalid image format:**
```
Error: Unsupported image format for 'file.xyz'
```
Solution: Use supported formats (JPG, PNG, GIF, BMP, WebP, TGA, DDS)

**Invalid grid dimensions:**
```
Error: Width and height must be positive integers
```
Solution: Use values > 0 for `-w` and `-h`

**Conflicting options:**
```
Error: Cannot use --adaptive with --width or --height
```
Solution: Use either fixed grid (-w, -h) or adaptive mode (--adaptive)

**Missing required parameters:**
```
Error: The following required arguments were not provided:
  --input <INPUT>
  --output <OUTPUT>
```
Solution: Provide both input and output file paths

### Memory Errors

**Out of memory:**
```
Error: Out of memory during processing
```
Solutions:
- Reduce grid size (-w, -h values)
- Use adaptive mode
- Close other applications
- Use average algorithm instead of k-means

**Image too large:**
```
Error: Input image exceeds maximum size limit
```
Solutions:
- Resize input image
- Use smaller grid dimensions
- Use adaptive mode with higher variance threshold

## Performance Considerations

### Speed Optimization

**Fastest processing:**
- Use `average` algorithm
- Use smaller grid sizes
- Avoid k-means for large images

**Memory optimization:**
- Use adaptive mode for large images
- Limit color count (lower values)
- Process images in batches

### Quality vs Performance

| Priority | Algorithm | Grid Size | Colors | Processing Time |
|----------|-----------|-----------|--------|-----------------|
| Speed | average | 16x16 | N/A | Fastest |
| Balanced | median-cut | 32x32 | 16 | Medium |
| Quality | kmeans | 64x64 | 32 | Slowest |

## Batch Processing

### Shell Scripts

**Bash example:**
```bash
#!/bin/bash
for img in *.jpg; do
    pixel-art-rust -w 32 -h 32 -i "$img" -o "pixel_${img%.*}.png"
done
```

**With different algorithms:**
```bash
for img in photos/*.jpg; do
    base=$(basename "$img" .jpg)
    pixel-art-rust -w 48 -h 48 -i "$img" -o "output/${base}_avg.png" --algorithm average
    pixel-art-rust -w 48 -h 48 -i "$img" -o "output/${base}_km.png" --algorithm kmeans --colors 16
done
```

### PowerShell Example

```powershell
Get-ChildItem *.jpg | ForEach-Object {
    $output = "pixel_" + $_.BaseName + ".png"
    pixel-art-rust -w 32 -h 32 -i $_.Name -o $output
}
```

## Configuration Files

Currently, Pixel Art Rust does not support configuration files. All options must be specified via command-line arguments.

## Environment Variables

No environment variables are currently supported. Use command-line arguments for all configuration.

## Logging and Debugging

Use the verbose flag for detailed output:
```bash
# Note: Verbose mode is planned for future releases
pixel-art-rust -v -w 32 -h 32 -i image.jpg -o output.png
```

## See Also

- [Usage Guide](/guide/usage) - Practical usage examples
- [Algorithm Details](/algorithms/overview) - Technical algorithm explanations
- [Core Library API](/api/core) - Programmatic API reference