# Color Quantization Algorithms

Pixel Art Rust implements several color quantization algorithms, each with its own strengths.

## Comparison

| Algorithm  | Speed   | Quality  | Memory | Use Case                       |
| ---------- | ------- | -------- | ------ | ------------------------------ |
| Average    | Fastest | Good     | Low    | Quick previews, uniform images |
| Median Cut | Fast    | Better   | Medium | Balanced quality/speed         |
| K-Means    | Slow    | Best     | High   | Maximum quality                |
| Quadtree   | Fast    | Adaptive | Medium | Images with varying detail     |

## Algorithm Details

### Average Color

Simple and fast - calculates the arithmetic mean of all pixels in each cell.

### Median Cut

Recursively divides the color space to create a balanced palette.

### K-Means Clustering

Iteratively refines color clusters for optimal representation.

### Adaptive Quadtree

Dynamically subdivides regions based on color variance.

## How Color Quantization Works

Color quantization is the process of reducing the number of distinct colors in an image. In pixel art conversion, this serves two purposes:

1. **Simplification**: Reduces visual complexity to achieve the characteristic pixel art aesthetic
2. **Regionalization**: Groups similar colors together to define pixel boundaries

## The Pixel Art Process

### Step 1: Grid Division

The image is divided into a grid of cells (or adaptive regions for quadtree).

### Step 2: Color Analysis

For each cell, the algorithm analyzes all contained pixels to determine the representative color.

### Step 3: Color Selection

Different algorithms use different strategies:

- **Average**: Simple arithmetic mean
- **Median Cut**: Optimal color space division
- **K-Means**: Iterative clustering
- **Quadtree**: Variance-based subdivision

### Step 4: Reconstruction

The final image is built by filling each cell with its representative color.

## Visual Quality Factors

### Color Accuracy

How well the chosen colors represent the original image content.

### Edge Preservation

How well the algorithm maintains important visual boundaries.

### Gradient Handling

How smoothly the algorithm handles color transitions.

### Detail Retention

How much fine detail is preserved in the conversion.

## Performance Characteristics

### Time Complexity

- **Average**: O(n) - fastest
- **Median Cut**: O(n log n) - moderate
- **K-Means**: O(n × k × i) - slowest
- **Quadtree**: O(n log d) - moderate

Where:

- n = number of pixels
- k = number of colors (for k-means)
- i = number of iterations
- d = maximum depth

### Memory Usage

- **Average**: Minimal - only stores running totals
- **Median Cut**: Moderate - stores color lists for division
- **K-Means**: High - maintains cluster assignments
- **Quadtree**: Variable - depends on image complexity

## Choosing the Right Algorithm

### For Speed (Real-time Processing)

Choose **Average Color** when you need the fastest possible conversion:

- Live previews while adjusting parameters
- Batch processing large numbers of images
- Resource-constrained environments

### For Quality (Final Output)

Choose **K-Means** when you want the best possible results:

- Final artwork creation
- Professional projects
- When processing time is not a concern

### For Balance (Most Common Use)

Choose **Median Cut** for the best compromise:

- General-purpose pixel art creation
- When you need good quality in reasonable time
- Most photography and artwork conversion

### For Adaptive Detail (Complex Images)

Choose **Quadtree** for images with varying complexity:

- Landscapes with both detailed and uniform areas
- Technical drawings with mixed content
- When you want automatic detail preservation

## Color Spaces

### RGB Color Space

- Standard computer graphics representation
- Simple calculations
- Not perceptually uniform

### LAB Color Space

- Perceptually uniform
- Better for color distance calculations
- Used internally for higher-quality algorithms

The library automatically handles color space conversions for optimal results with each algorithm.

## Algorithm Limitations

### Average Color

- Can produce muddy colors in diverse regions
- No consideration of color distribution
- May lose important color relationships

### Median Cut

- Limited by binary division strategy
- May not find optimal global palette
- Can struggle with very sparse color distributions

### K-Means

- Requires multiple iterations to converge
- Can get stuck in local optima
- Sensitive to initial centroid placement

### Quadtree

- Recursive subdivision may miss optimal boundaries
- Variance threshold selection affects results
- Can create irregular pixel shapes

## Future Improvements

The algorithm implementations are continuously improved. Planned enhancements include:

- **SIMD optimizations** for parallel color processing
- **GPU acceleration** for large image processing
- **Perceptual weighting** for better visual results
- **Custom color palettes** for specific aesthetic goals

## Technical Resources

For implementation details and API documentation, see:

- [Algorithm API Reference](/api/algorithms) - Technical implementation details
- [Core Library API](/api/core) - High-level programming interface
- [CLI Reference](/api/cli) - Command-line usage and options

For practical usage examples:

- [Usage Guide](/guide/usage) - Detailed usage patterns
- [Examples](/guide/examples) - Real-world conversion examples
