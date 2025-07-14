# Usage Guide

This comprehensive guide covers all aspects of using Pixel Art Rust effectively.

## Basic Usage

The simplest way to convert an image to pixel art:

```bash
pixel-art-rust -w 32 -h 32 -i input.jpg -o output.png
```

This creates a 32x32 pixel grid using the default average color algorithm.

## Command Line Options

### Required Parameters

- `-i, --input <PATH>` - Input image path (supports JPG, PNG, GIF, BMP, WebP)
- `-o, --output <PATH>` - Output image path (PNG recommended for best quality)

### Grid Configuration

**Fixed Grid Mode:**
- `-w, --width <WIDTH>` - Number of horizontal divisions
- `-h, --height <HEIGHT>` - Number of vertical divisions

**Adaptive Mode:**
- `--adaptive` - Use adaptive quadtree instead of uniform grid
- `--max-depth <DEPTH>` - Maximum quadtree depth (default: 10)
- `--variance-threshold <VAL>` - Color variance threshold for splitting (default: 50.0)

### Algorithm Selection

- `-a, --algorithm <ALGORITHM>` - Color extraction algorithm
  - `average` (default) - Fast arithmetic mean
  - `median-cut` - Balanced quality/speed
  - `kmeans` - Highest quality
- `-c, --colors <COLORS>` - Number of colors for quantization (used with median-cut and kmeans)

## Usage Patterns

### Quick Preview
For fast previews while experimenting:
```bash
pixel-art-rust -w 16 -h 16 -i photo.jpg -o preview.png
```

### High Quality Output
For final artwork with best quality:
```bash
pixel-art-rust -w 64 -h 64 -i photo.jpg -o final.png --algorithm kmeans --colors 32
```

### Adaptive Detail
For images with varying levels of detail:
```bash
pixel-art-rust --adaptive -i landscape.jpg -o adaptive.png --max-depth 8
```

### Specific Color Palettes
Limit colors for retro aesthetics:
```bash
pixel-art-rust -w 48 -h 48 -i portrait.jpg -o retro.png --algorithm median-cut --colors 8
```

## Algorithm Selection Guide

### When to Use Average
- **Best for:** Quick previews, uniform images, batch processing
- **Characteristics:** Fastest, good for images with consistent lighting
- **Example:** Screenshots, logos, simple graphics

### When to Use Median Cut
- **Best for:** Balanced results, moderate processing time
- **Characteristics:** Good color distribution, handles gradients well
- **Example:** Photographs with varied colors

### When to Use K-Means
- **Best for:** Highest quality output, final artwork
- **Characteristics:** Optimal color clustering, slower processing
- **Example:** Portraits, detailed artwork, professional projects

### When to Use Adaptive Quadtree
- **Best for:** Images with varying detail levels
- **Characteristics:** Preserves detail where needed, efficient compression
- **Example:** Landscapes, architectural photos, mixed content

## Image Format Considerations

### Input Formats
- **JPG/JPEG:** Good for photographs
- **PNG:** Best for images with transparency or sharp edges
- **GIF:** Acceptable but limited color depth
- **BMP:** Uncompressed, large files
- **WebP:** Modern format, good compression

### Output Recommendations
- **PNG:** Recommended for all pixel art (lossless, good compression)
- **JPG:** Avoid due to compression artifacts
- **GIF:** Only if you need animation or extremely small files

## Performance Tips

### For Large Images
```bash
# Use lower resolution first to test
pixel-art-rust -w 32 -h 32 -i large_image.jpg -o test.png

# Then upscale for final version
pixel-art-rust -w 128 -h 128 -i large_image.jpg -o final.png --algorithm average
```

### Batch Processing
```bash
# Process multiple images with a script
for img in *.jpg; do
    pixel-art-rust -w 32 -h 32 -i "$img" -o "pixel_${img%.*}.png"
done
```

### Memory Management
- Use smaller grid sizes for very large input images
- Consider the adaptive mode for memory efficiency
- Monitor system resources during processing

## Common Workflows

### Photo to Pixel Art
1. Start with average algorithm for quick preview
2. Adjust grid size to balance detail and pixel aesthetic
3. Switch to k-means for final high-quality output
4. Fine-tune color count if needed

### Logo Pixelation
1. Use small grid sizes (16x16 or 24x24)
2. Average algorithm works well for simple graphics
3. Ensure output maintains readability

### Game Asset Creation
1. Use consistent grid sizes across related assets
2. Median-cut algorithm for good color balance
3. Limit color count to match your game's palette

## Troubleshooting

### Common Issues

**Output too blurry:**
- Increase grid resolution (-w and -h values)
- Try median-cut or k-means algorithms
- Reduce color count for sharper edges

**Processing too slow:**
- Use average algorithm for faster processing
- Reduce grid resolution
- Consider adaptive mode for large images

**Colors look wrong:**
- Try different algorithms (k-means often has best colors)
- Adjust color count parameter
- Check input image quality

**File size too large:**
- Use PNG format for better compression
- Consider reducing output dimensions
- Limit color count with quantization algorithms

## Next Steps

- Explore specific [examples](/guide/examples) with different image types
- Learn about the [algorithm details](/algorithms/overview)
- Check out the [API reference](/api/cli) for programmatic usage