# Median Cut Algorithm

The Median Cut algorithm is a classic color quantization technique that creates balanced color palettes by recursively dividing the color space. It provides a good balance between quality and performance.

## How It Works

### Basic Concept

The algorithm works by:

1. Starting with all colors in the image
2. Finding the color channel (R, G, or B) with the largest range
3. Sorting colors by that channel and splitting at the median
4. Recursively applying this process to each half
5. Continuing until the desired number of colors is reached

### Step-by-Step Process

**Step 1: Color Collection**

```
Original colors: [Red, Green, Blue, Yellow, Purple, Orange, ...]
```

**Step 2: Find Longest Axis**

```
Red range:   0-255 (range: 255)
Green range: 50-200 (range: 150)
Blue range:  10-180 (range: 170)

Longest axis: Red (255)
```

**Step 3: Sort and Split**

```
Sorted by red: [Blue, Green, Purple, Orange, Yellow, Red]
Median split: [Blue, Green, Purple] | [Orange, Yellow, Red]
```

**Step 4: Recursive Division**
Each group is further divided until reaching the target color count.

## Visual Example

Consider reducing 8 colors to 4 colors:

### Initial Color Set

```
Colors: Red, Orange, Yellow, Green, Cyan, Blue, Purple, Pink
```

### First Split (by Red channel)

```
Group A: Blue, Cyan, Green, Purple (low red values)
Group B: Red, Orange, Yellow, Pink (high red values)
```

### Second Split (Group A by Blue channel)

```
Group A1: Green (low blue)
Group A2: Blue, Cyan, Purple (high blue)
```

### Second Split (Group B by Green channel)

```
Group B1: Red, Purple (low green)
Group B2: Orange, Yellow, Pink (high green)
```

### Final Representatives

```
Group A1 → Green
Group A2 → Blue
Group B1 → Red
Group B2 → Orange
```

Result: 4 representative colors that span the original color space effectively.

## Algorithm Details

### Color Space Analysis

The algorithm analyzes the distribution of colors in 3D RGB space:

```
For each color channel (R, G, B):
    min_value = minimum value in current color set
    max_value = maximum value in current color set
    range = max_value - min_value

longest_axis = channel with maximum range
```

### Median Calculation

Finding the median ensures balanced splits:

```
sorted_colors = sort colors by longest_axis value
median_index = length(sorted_colors) / 2
split_point = sorted_colors[median_index]

left_group = colors with values ≤ split_point
right_group = colors with values > split_point
```

### Recursion Control

The process continues until reaching the target number of groups:

```
target_colors = 16  // User-specified
current_groups = 1  // Start with all colors

while current_groups < target_colors:
    select group with largest color range
    split that group at median
    current_groups += 1
```

## Advantages

### Balanced Color Distribution

- **Even coverage**: Each final color represents roughly equal portions of the color space
- **No bias**: Algorithm doesn't favor any particular colors
- **Comprehensive**: Covers the full range of colors in the image

### Good Quality Results

- **Preserves contrast**: Maintains color relationships better than averaging
- **Handles gradients**: Creates smooth transitions between similar colors
- **Detail retention**: Important color distinctions are preserved

### Predictable Performance

- **Consistent timing**: Performance is predictable based on target color count
- **Reasonable speed**: Faster than iterative methods like K-means
- **Memory efficient**: Doesn't require large intermediate data structures

## Disadvantages

### Limited by Binary Division

- **Suboptimal splits**: May not find the best possible division points
- **Linear boundaries**: Can only create axis-aligned splits in color space
- **Missing optimal palettes**: Global optimum may require non-binary divisions

### Color Space Limitations

- **RGB bias**: Works in RGB space which isn't perceptually uniform
- **Channel dependency**: Results depend on which color channel varies most
- **Ignores perception**: Doesn't account for human color perception differences

### Quality Limitations

- **Local optimization**: Each split is locally optimal but may not be globally optimal
- **Sparse colors**: May struggle with images having very few distinct colors
- **Outlier sensitivity**: Unusual colors can skew the splitting process

## Best Use Cases

### General Photography

Perfect for converting photographs to pixel art:

```bash
pixel-art-rust -w 48 -h 36 -i landscape.jpg -o pixel_landscape.png \
  --algorithm median-cut --colors 32
```

### Balanced Quality/Speed

When you need good results in reasonable time:

```bash
# Good for interactive applications
pixel-art-rust -w 64 -h 64 -i portrait.jpg -o result.png \
  --algorithm median-cut --colors 16
```

### Artwork with Varied Colors

Works well with images containing diverse color palettes:

- Digital paintings
- Colorful illustrations
- Nature photography
- Abstract art

### Batch Processing

Reliable results across different image types:

```bash
for img in photos/*.jpg; do
    name=$(basename "$img" .jpg)
    pixel-art-rust -w 32 -h 32 -i "$img" -o "output/${name}_pixel.png" \
      --algorithm median-cut --colors 24
done
```

## Configuration Options

### Color Count Selection

The `--colors` parameter significantly affects results:

**Low color counts (4-8):**

- Highly stylized results
- Strong posterization effect
- Good for retro/vintage aesthetics

**Medium color counts (12-24):**

- Balanced detail and simplification
- Good general-purpose setting
- Preserves most important color relationships

**High color counts (32-64):**

- Detailed color representation
- Subtle posterization
- Good for high-quality conversions

### Examples by Color Count

```bash
# Retro 8-bit style
pixel-art-rust -i photo.jpg -o retro.png -w 32 -h 32 \
  --algorithm median-cut --colors 8

# Balanced conversion
pixel-art-rust -i photo.jpg -o balanced.png -w 48 -h 48 \
  --algorithm median-cut --colors 16

# High quality
pixel-art-rust -i photo.jpg -o detailed.png -w 64 -h 64 \
  --algorithm median-cut --colors 32
```

## Performance Characteristics

### Time Complexity: O(n log c)

- n = number of unique colors in image
- c = target color count
- Each split requires sorting, which is O(n log n)
- Total of log(c) splits needed

### Space Complexity: O(n)

- Stores list of colors for sorting and splitting
- Temporary storage for color groups
- Minimal overhead beyond input data

### Benchmarks

Typical processing times:

| Image Size | Colors | Grid Size | Time  |
| ---------- | ------ | --------- | ----- |
| 1024×1024  | 16     | 32×32     | 0.05s |
| 1024×1024  | 32     | 48×48     | 0.08s |
| 2048×2048  | 16     | 64×64     | 0.15s |
| 2048×2048  | 32     | 64×64     | 0.25s |

## Implementation Considerations

### Color Space Choice

While traditionally implemented in RGB, LAB color space provides better results:

**RGB Space:**

- Fast computation
- Simple implementation
- Not perceptually uniform

**LAB Space:**

- Perceptually uniform
- Better color distance calculations
- Slightly slower computation

### Optimization Techniques

**Pre-sorting:** Sort colors once, maintain sorted order through splits:

```
initial_sort = sort_all_colors_by_luminance()
// Subsequent sorts only need to reorder within groups
```

**Variance tracking:** Track color variance to prioritize splits:

```
split_priority = group_with_highest_variance()
// Focus computation on groups that benefit most from splitting
```

**Early termination:** Stop splitting when variance is low:

```
if color_variance < threshold:
    use_average_color()  // No benefit from further splitting
```

## Comparison with Other Algorithms

### vs. Average Color

- **Quality**: Median Cut produces much better color palettes
- **Speed**: ~3-5x slower than average
- **Memory**: Uses more memory for color storage
- **Use case**: Choose Median Cut when quality matters more than speed

### vs. K-Means

- **Quality**: K-Means often produces superior results
- **Speed**: Median Cut is ~3-5x faster
- **Determinism**: Median Cut is deterministic, K-Means can vary
- **Use case**: Median Cut for consistent, fast results

### vs. Quadtree

- **Approach**: Different focus - Median Cut for color, Quadtree for spatial
- **Quality**: Comparable for different aspects
- **Speed**: Similar performance
- **Use case**: Median Cut for color-focused images, Quadtree for spatial detail

## Common Issues and Solutions

### Poor Color Selection

**Problem**: Important colors are missing from the palette

**Solution**: Increase color count or use K-Means for final output

```bash
# Try more colors first
pixel-art-rust -i photo.jpg -o test.png -w 48 -h 48 \
  --algorithm median-cut --colors 32

# Or switch to K-Means for quality
pixel-art-rust -i photo.jpg -o quality.png -w 48 -h 48 \
  --algorithm kmeans --colors 16
```

### Oversaturated Results

**Problem**: Colors appear too vivid or unnatural

**Solution**: Post-process to reduce saturation

```bash
# Reduce saturation after conversion
convert output.png -modulate 100,80,100 final.png
```

### Loss of Detail

**Problem**: Important details disappear in conversion

**Solution**: Increase grid resolution or switch algorithms

```bash
# Higher resolution grid
pixel-art-rust -i detailed.jpg -o output.png -w 96 -h 72 \
  --algorithm median-cut --colors 24

# Or try adaptive approach
pixel-art-rust -i detailed.jpg -o adaptive.png --adaptive \
  --max-depth 8 --variance-threshold 25.0
```

## Advanced Techniques

### Hierarchical Processing

Use different algorithms at different scales:

1. **Coarse level**: Use Median Cut for overall color palette
2. **Fine level**: Use Average for individual cell colors
3. **Refinement**: Apply local color adjustments

### Custom Color Spaces

Implement Median Cut in perceptually uniform color spaces:

- **LAB space**: Better perceptual uniformity
- **HSV space**: Better for artistic control
- **LUV space**: Alternative perceptual space

### Weighted Splitting

Consider pixel frequency when splitting:

```
Instead of geometric median:
weighted_median = consider pixel count in each color

More frequent colors get higher priority in splits
```

## Example Workflows

### Portrait Photography

```bash
# Good for skin tones and facial features
pixel-art-rust -i portrait.jpg -o pixel_portrait.png -w 64 -h 80 \
  --algorithm median-cut --colors 24
```

### Landscape Photography

```bash
# Handles sky gradients and varied terrain well
pixel-art-rust -i landscape.jpg -o pixel_landscape.png -w 96 -h 54 \
  --algorithm median-cut --colors 32
```

### Artistic Illustrations

```bash
# Preserves color relationships in artwork
pixel-art-rust -i artwork.png -o pixel_art.png -w 80 -h 80 \
  --algorithm median-cut --colors 16
```

### Retro Gaming Style

```bash
# Limited palette for authentic retro feel
pixel-art-rust -i modern.jpg -o retro.png -w 32 -h 32 \
  --algorithm median-cut --colors 8
```

## See Also

- [K-Means Algorithm](/algorithms/kmeans) - Higher quality alternative
- [Average Color Algorithm](/algorithms/average-color) - Faster alternative
- [Quadtree Algorithm](/algorithms/quadtree) - Spatial detail preservation
- [Algorithm Overview](/algorithms/overview) - Complete algorithm comparison
