# Adaptive Quadtree Algorithm

The Adaptive Quadtree algorithm provides dynamic spatial subdivision, creating larger pixels in uniform areas and smaller pixels where detail is needed. This results in efficient representation that preserves important visual information.

## How It Works

### Basic Concept

Unlike fixed-grid approaches, the quadtree algorithm:
1. Starts with the entire image as one region
2. Analyzes color variance in the region
3. If variance exceeds a threshold, splits into 4 sub-regions
4. Recursively applies this process to each sub-region
5. Stops when variance is low or maximum depth is reached
6. Uses the average color for each final region

### Visual Example

Consider a landscape image with sky and detailed foreground:

**Level 0 (Full Image):**
```
┌─────────────────────────┐
│ High variance detected  │
│ Split into 4 quadrants  │
└─────────────────────────┘
```

**Level 1 (4 Regions):**
```
┌───────────┬─────────────┐
│ Sky       │ Sky         │
│ Low var.  │ Low var.    │
├───────────┼─────────────┤
│ Trees     │ Mountain    │
│ High var. │ Med var.    │
└───────────┴─────────────┘
```

**Level 2 (Selective Splitting):**
```
┌───────────┬─────────────┐
│           │             │
│    Sky    │    Sky      │
│           │             │
├─────┬─────┼─────────────┤
│Tree │Tree │             │
│ A   │ B   │  Mountain   │
├─────┼─────┤             │
│Tree │Tree │             │
│ C   │ D   │             │
└─────┴─────┴─────────────┘
```

**Final Result:**
- Sky: 2 large pixels (low detail)
- Trees: 4 small pixels (high detail)  
- Mountain: 1 medium pixel (medium detail)

## Algorithm Steps

### Step 1: Variance Calculation

For each region, calculate color variance:

```rust
fn calculate_variance(pixels: &[Color]) -> f32 {
    if pixels.len() <= 1 {
        return 0.0;
    }
    
    let mean = average_color(pixels);
    let sum_squared_diff: f32 = pixels.iter()
        .map(|color| color_distance_squared(color, &mean))
        .sum();
    
    sum_squared_diff / pixels.len() as f32
}
```

### Step 2: Split Decision

Decide whether to split based on variance and depth:

```rust
fn should_split(
    variance: f32, 
    depth: usize, 
    max_depth: usize, 
    variance_threshold: f32
) -> bool {
    depth < max_depth && variance > variance_threshold
}
```

### Step 3: Quadrant Division

Split region into 4 equal sub-regions:

```rust
fn split_region(bounds: Rectangle) -> [Rectangle; 4] {
    let mid_x = bounds.x + bounds.width / 2;
    let mid_y = bounds.y + bounds.height / 2;
    
    [
        Rectangle::new(bounds.x, bounds.y, mid_x, mid_y),           // Top-left
        Rectangle::new(mid_x, bounds.y, bounds.right(), mid_y),    // Top-right
        Rectangle::new(bounds.x, mid_y, mid_x, bounds.bottom()),   // Bottom-left
        Rectangle::new(mid_x, mid_y, bounds.right(), bounds.bottom()), // Bottom-right
    ]
}
```

### Step 4: Recursive Processing

Apply the algorithm recursively:

```rust
fn build_quadtree(
    image: &Image,
    bounds: Rectangle,
    depth: usize,
    max_depth: usize,
    variance_threshold: f32,
) -> QuadtreeNode {
    let pixels = extract_pixels(image, bounds);
    let variance = calculate_variance(&pixels);
    
    if should_split(variance, depth, max_depth, variance_threshold) {
        let children = split_region(bounds)
            .map(|child_bounds| {
                Box::new(build_quadtree(
                    image, 
                    child_bounds, 
                    depth + 1, 
                    max_depth, 
                    variance_threshold
                ))
            });
        
        QuadtreeNode {
            bounds,
            color: None,
            children: Some(children),
            variance,
        }
    } else {
        QuadtreeNode {
            bounds,
            color: Some(average_color(&pixels)),
            children: None,
            variance,
        }
    }
}
```

## Advantages

### Adaptive Detail Preservation
- **Smart allocation**: More detail where needed, less where uniform
- **Efficient representation**: Minimal subdivision in simple areas
- **Natural boundaries**: Splits tend to follow image features

### Memory Efficiency
- **Compact representation**: Fewer regions than fixed grid for many images
- **Scalable quality**: Can handle very high detail without excessive memory
- **Efficient storage**: Tree structure naturally compresses uniform areas

### Visual Quality
- **Preserves edges**: Detail retention along important boundaries
- **Smooth areas**: Large pixels in uniform regions look natural
- **Balanced complexity**: Automatic trade-off between detail and simplification

## Disadvantages

### Irregular Pixel Shapes
- **Non-uniform appearance**: Variable pixel sizes may look inconsistent
- **Not traditionally "pixel art"**: Doesn't match classic fixed-grid aesthetic
- **Complex boundaries**: Irregular shapes can look messy in some contexts

### Parameter Sensitivity
- **Threshold tuning**: Variance threshold greatly affects results
- **Depth selection**: Maximum depth impacts quality vs. complexity
- **No clear guidelines**: Optimal parameters vary by image type

### Processing Complexity
- **Recursive overhead**: Tree traversal adds computational cost
- **Memory fragmentation**: Dynamic allocation can be less cache-friendly
- **Implementation complexity**: More complex than fixed-grid approaches

## Configuration Parameters

### Maximum Depth

Controls the smallest possible pixel size:

**Low depth (4-6):**
```bash
pixel-art-rust --adaptive -i image.jpg -o coarse.png --max-depth 4
```
- Larger minimum pixel size
- Faster processing
- Less detail preservation
- Good for highly stylized results

**Medium depth (8-10):**
```bash
pixel-art-rust --adaptive -i image.jpg -o balanced.png --max-depth 8
```
- Balanced detail and simplification
- Reasonable processing time
- Good general-purpose setting

**High depth (12-16):**
```bash
pixel-art-rust --adaptive -i image.jpg -o detailed.png --max-depth 12
```
- Very fine detail possible
- Slower processing
- May create many small pixels
- Good for high-quality conversions

### Variance Threshold

Controls sensitivity to color variation:

**Low threshold (10-25):**
```bash
pixel-art-rust --adaptive -i image.jpg -o sensitive.png \
  --max-depth 8 --variance-threshold 15.0
```
- More sensitive to color changes
- Creates more subdivisions
- Preserves subtle details
- May oversegment uniform areas

**Medium threshold (30-50):**
```bash
pixel-art-rust --adaptive -i image.jpg -o balanced.png \
  --max-depth 8 --variance-threshold 40.0
```
- Balanced sensitivity
- Good for most image types
- Reasonable subdivision count

**High threshold (60-100):**
```bash
pixel-art-rust --adaptive -i image.jpg -o coarse.png \
  --max-depth 8 --variance-threshold 75.0
```
- Less sensitive to variations
- Creates fewer subdivisions
- More stylized results
- Good for abstract effects

## Performance Characteristics

### Time Complexity: O(n log d)
- n = number of pixels
- d = maximum depth
- Each pixel is processed at most d times (once per level)

### Space Complexity: O(4^d)
- Worst case: complete tree with all nodes subdivided
- Typical case: Much smaller due to early termination
- Best case: O(1) for completely uniform images

### Benchmarks

Typical processing times:

| Image Size | Max Depth | Threshold | Regions | Time    |
|------------|-----------|-----------|---------|---------|
| 1024×1024  | 8         | 40.0      | 256     | 0.15s   |
| 1024×1024  | 10        | 25.0      | 512     | 0.25s   |
| 2048×2048  | 8         | 40.0      | 384     | 0.45s   |
| 2048×2048  | 12        | 15.0      | 1024    | 1.2s    |

## Best Use Cases

### Landscape Photography
Excellent for images with varied detail levels:
```bash
pixel-art-rust --adaptive -i landscape.jpg -o adaptive_landscape.png \
  --max-depth 10 --variance-threshold 30.0
```

### Architectural Images
Good for buildings with both detailed and smooth areas:
```bash
pixel-art-rust --adaptive -i building.jpg -o adaptive_building.png \
  --max-depth 12 --variance-threshold 35.0
```

### Mixed Content Images
Perfect for images combining simple and complex regions:
```bash
pixel-art-rust --adaptive -i mixed.jpg -o adaptive_mixed.png \
  --max-depth 8 --variance-threshold 45.0
```

### Large Images with Detail
Efficient for high-resolution images:
```bash
pixel-art-rust --adaptive -i 4k_photo.jpg -o adaptive_4k.png \
  --max-depth 14 --variance-threshold 25.0
```

## Color Space Considerations

### RGB vs. LAB Space

**RGB Space:**
- Faster computation
- May not represent perceptual differences well
- Can lead to poor splitting decisions

**LAB Space (Recommended):**
- Perceptually uniform
- Better variance calculations  
- More natural region boundaries
- Slightly slower computation

### Variance Calculation Methods

**Simple RGB Variance:**
```rust
fn rgb_variance(colors: &[RGB]) -> f32 {
    let mean = average_rgb(colors);
    colors.iter()
        .map(|c| {
            let dr = c.r as f32 - mean.r as f32;
            let dg = c.g as f32 - mean.g as f32;
            let db = c.b as f32 - mean.b as f32;
            dr*dr + dg*dg + db*db
        })
        .sum::<f32>() / colors.len() as f32
}
```

**Perceptual LAB Variance:**
```rust
fn lab_variance(colors: &[LAB]) -> f32 {
    let mean = average_lab(colors);
    colors.iter()
        .map(|c| delta_e_2000(c, &mean).powi(2))
        .sum::<f32>() / colors.len() as f32
}
```

## Advanced Techniques

### Weighted Variance
Consider pixel importance when calculating variance:

```rust
fn weighted_variance(colors: &[Color], weights: &[f32]) -> f32 {
    let weighted_mean = calculate_weighted_average(colors, weights);
    let weighted_sum: f32 = colors.iter()
        .zip(weights.iter())
        .map(|(color, weight)| {
            weight * color_distance_squared(color, &weighted_mean)
        })
        .sum();
    
    weighted_sum / weights.iter().sum::<f32>()
}
```

### Directional Splitting
Split along edges rather than geometric center:

```rust
fn edge_aware_split(region: Rectangle, edge_map: &EdgeMap) -> [Rectangle; 4] {
    let split_x = find_vertical_edge_in_region(region, edge_map);
    let split_y = find_horizontal_edge_in_region(region, edge_map);
    
    // Split at edge locations rather than geometric center
    create_quadrants(region, split_x, split_y)
}
```

### Multi-Scale Analysis
Consider variance at multiple scales:

```rust
fn multi_scale_variance(
    pixels: &[Color], 
    region: Rectangle
) -> f32 {
    let local_variance = calculate_variance(pixels);
    let neighbor_variance = calculate_neighborhood_variance(region);
    let gradient_variance = calculate_gradient_variance(region);
    
    // Combine multiple variance measures
    0.6 * local_variance + 0.3 * neighbor_variance + 0.1 * gradient_variance
}
```

## Common Issues and Solutions

### Over-Segmentation
**Problem**: Too many small regions, looks fragmented

**Solutions:**
- Increase variance threshold
- Reduce maximum depth
- Use larger minimum region size

```bash
# Reduce segmentation
pixel-art-rust --adaptive -i over_segmented.jpg -o simplified.png \
  --max-depth 6 --variance-threshold 60.0
```

### Under-Segmentation
**Problem**: Important details are lost, looks too simple

**Solutions:**
- Decrease variance threshold
- Increase maximum depth
- Consider perceptual color space

```bash
# Increase detail preservation
pixel-art-rust --adaptive -i under_segmented.jpg -o detailed.png \
  --max-depth 12 --variance-threshold 20.0
```

### Irregular Appearance
**Problem**: Results don't look like traditional pixel art

**Solutions:**
- Use fixed grid for consistent appearance
- Post-process to regularize pixel shapes
- Adjust parameters for more uniform regions

```bash
# More uniform appearance
pixel-art-rust --adaptive -i irregular.jpg -o uniform.png \
  --max-depth 8 --variance-threshold 80.0
```

### Poor Boundary Alignment
**Problem**: Splits don't align with image features

**Solutions:**
- Use edge-aware splitting
- Preprocess with edge enhancement
- Try different color spaces

```bash
# Enhance edges before processing
convert input.jpg -edge 1 -negate -blur 0x1 edge_enhanced.jpg
pixel-art-rust --adaptive -i edge_enhanced.jpg -o aligned.png \
  --max-depth 10 --variance-threshold 35.0
```

## Comparison with Other Algorithms

### vs. Fixed Grid (Average)
- **Adaptivity**: Quadtree adapts to content, fixed grid is uniform
- **Efficiency**: Quadtree can be more efficient for simple images
- **Aesthetic**: Fixed grid has traditional pixel art look
- **Use case**: Quadtree for efficiency, fixed grid for consistency

### vs. Fixed Grid (K-Means)
- **Quality**: K-Means optimizes colors, quadtree optimizes spatial layout
- **Speed**: Comparable performance
- **Results**: Different strengths - color vs. spatial optimization
- **Use case**: K-Means for color-rich images, quadtree for spatial detail

### vs. Median Cut
- **Approach**: Both use recursive subdivision but in different spaces
- **Quality**: Comparable but for different aspects
- **Speed**: Similar performance characteristics
- **Use case**: Median cut for color palette, quadtree for spatial efficiency

## Example Workflows

### Nature Photography
```bash
# Landscapes with sky, water, and detailed terrain
pixel-art-rust --adaptive -i nature.jpg -o adaptive_nature.png \
  --max-depth 10 --variance-threshold 30.0
```

### Urban Scenes
```bash
# Buildings with both detailed and smooth areas
pixel-art-rust --adaptive -i cityscape.jpg -o adaptive_city.png \
  --max-depth 12 --variance-threshold 40.0
```

### Portrait Photography
```bash
# Faces with detailed features and smooth skin
pixel-art-rust --adaptive -i portrait.jpg -o adaptive_portrait.png \
  --max-depth 8 --variance-threshold 25.0
```

### Technical Drawings
```bash
# Diagrams with lines and uniform areas
pixel-art-rust --adaptive -i diagram.png -o adaptive_diagram.png \
  --max-depth 14 --variance-threshold 15.0
```

### Abstract Art
```bash
# Complex patterns with varying detail
pixel-art-rust --adaptive -i abstract.jpg -o adaptive_abstract.png \
  --max-depth 10 --variance-threshold 50.0
```

## Parameter Tuning Guide

### Start with Defaults
```bash
pixel-art-rust --adaptive -i image.jpg -o test.png \
  --max-depth 8 --variance-threshold 40.0
```

### Adjust for More Detail
```bash
# If result is too simple
pixel-art-rust --adaptive -i image.jpg -o detailed.png \
  --max-depth 10 --variance-threshold 25.0
```

### Adjust for Simplification
```bash
# If result is too complex
pixel-art-rust --adaptive -i image.jpg -o simple.png \
  --max-depth 6 --variance-threshold 60.0
```

### Fine-Tune for Quality
```bash
# Iteratively adjust based on results
pixel-art-rust --adaptive -i image.jpg -o v1.png \
  --max-depth 9 --variance-threshold 35.0

pixel-art-rust --adaptive -i image.jpg -o v2.png \
  --max-depth 9 --variance-threshold 30.0
```

## See Also

- [Average Color Algorithm](/algorithms/average-color) - Fixed-grid alternative
- [K-Means Algorithm](/algorithms/kmeans) - Color optimization focus
- [Median Cut Algorithm](/algorithms/median-cut) - Color space subdivision
- [Algorithm Overview](/algorithms/overview) - Complete algorithm comparison