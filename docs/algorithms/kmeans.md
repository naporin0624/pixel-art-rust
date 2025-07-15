# K-Means Clustering Algorithm

K-Means is an iterative clustering algorithm that produces the highest quality color palettes by finding optimal color groupings. It's the best choice when visual quality is the primary concern.

## How It Works

### Basic Concept

K-Means clustering works by:

1. Choosing K initial "cluster centers" (representative colors)
2. Assigning each pixel to the nearest cluster center
3. Updating cluster centers to the average of their assigned pixels
4. Repeating steps 2-3 until convergence
5. Using final cluster centers as the pixel art palette

### Visual Example

Converting 100 colors to 4 clusters:

**Initial State:**

```
Colors: [Various reds, blues, greens, yellows scattered in color space]
Initial centers: [Random Red, Random Blue, Random Green, Random Yellow]
```

**Iteration 1:**

```
Assign each color to nearest center:
- Reds, oranges → Red center
- Blues, purples → Blue center
- Greens → Green center
- Yellows → Yellow center

Update centers to average of assigned colors:
- Red center → Average of all reds/oranges
- Blue center → Average of all blues/purples
- Green center → Average of all greens
- Yellow center → Average of all yellows
```

**Iteration 2:**

```
Re-assign colors based on new centers:
- Some oranges might now be closer to Yellow center
- Some purples might now be closer to Red center

Update centers again...
```

**Convergence:**

```
Centers stop moving significantly
Final palette: [Optimized Red, Optimized Blue, Optimized Green, Optimized Yellow]
```

## Algorithm Steps

### Step 1: Initialization

**Random Initialization:**

```rust
// Simple but can lead to poor results
let mut centroids = Vec::new();
for _ in 0..k {
    centroids.push(random_color_from_image());
}
```

**K-Means++ Initialization (Better):**

```rust
// Choose first centroid randomly
centroids.push(random_color());

// Choose subsequent centroids with probability proportional to squared distance
for _ in 1..k {
    let distances: Vec<f32> = colors.iter()
        .map(|color| min_distance_to_existing_centroids(color, &centroids))
        .collect();

    let new_centroid = weighted_random_selection(colors, distances);
    centroids.push(new_centroid);
}
```

### Step 2: Assignment

For each pixel, find the closest centroid:

```rust
fn assign_to_clusters(colors: &[Color], centroids: &[Color]) -> Vec<usize> {
    colors.iter()
        .map(|color| {
            centroids.iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    color_distance(color, a).partial_cmp(&color_distance(color, b)).unwrap()
                })
                .unwrap()
                .0
        })
        .collect()
}
```

### Step 3: Update

Calculate new centroid positions:

```rust
fn update_centroids(colors: &[Color], assignments: &[usize], k: usize) -> Vec<Color> {
    let mut new_centroids = vec![Color::default(); k];
    let mut counts = vec![0; k];

    for (color, &cluster) in colors.iter().zip(assignments.iter()) {
        new_centroids[cluster] = new_centroids[cluster] + *color;
        counts[cluster] += 1;
    }

    for (centroid, count) in new_centroids.iter_mut().zip(counts.iter()) {
        if *count > 0 {
            *centroid = *centroid / (*count as f32);
        }
    }

    new_centroids
}
```

### Step 4: Convergence Check

```rust
fn has_converged(old_centroids: &[Color], new_centroids: &[Color], threshold: f32) -> bool {
    old_centroids.iter()
        .zip(new_centroids.iter())
        .all(|(old, new)| color_distance(old, new) < threshold)
}
```

## Advantages

### Optimal Color Selection

- **Global optimization**: Finds colors that minimize overall color distortion
- **Balanced clusters**: Each color represents roughly equal numbers of pixels
- **Natural groupings**: Tends to find perceptually meaningful color groups

### Superior Visual Quality

- **Smooth gradients**: Excellent handling of color transitions
- **Color harmony**: Produces aesthetically pleasing palettes
- **Detail preservation**: Important color relationships are maintained

### Flexibility

- **Configurable**: Many parameters can be tuned for specific needs
- **Adaptable**: Works well with various types of images
- **Scalable**: Can produce any number of colors from 2 to 256+

## Disadvantages

### Computational Complexity

- **Slow convergence**: May require many iterations
- **High memory usage**: Stores cluster assignments for all pixels
- **CPU intensive**: Distance calculations for every pixel-centroid pair

### Non-Deterministic Results

- **Random initialization**: Different runs can produce different results
- **Local optima**: May get stuck in suboptimal solutions
- **Sensitivity**: Results depend on initial centroid placement

### Parameter Sensitivity

- **Color count selection**: Wrong K value can produce poor results
- **Convergence threshold**: Affects quality vs. speed trade-off
- **Maximum iterations**: May stop before optimal solution

## Configuration Parameters

### Number of Colors (K)

**Low K (4-8 colors):**

```bash
pixel-art-rust -i photo.jpg -o minimal.png -w 32 -h 32 \
  --algorithm kmeans --colors 6
```

- Highly stylized results
- Strong artistic effect
- Good for logos and simple graphics

**Medium K (12-24 colors):**

```bash
pixel-art-rust -i portrait.jpg -o balanced.png -w 48 -h 48 \
  --algorithm kmeans --colors 16
```

- Good balance of detail and simplification
- Suitable for most photography
- Recommended for general use

**High K (32-64 colors):**

```bash
pixel-art-rust -i landscape.jpg -o detailed.png -w 80 -h 60 \
  --algorithm kmeans --colors 48
```

- High detail preservation
- Subtle artistic effect
- Good for high-quality conversions

### Internal Parameters

While not exposed in the CLI, the algorithm uses these parameters internally:

**Max Iterations:** Default 100

- Higher values: Better convergence, slower processing
- Lower values: Faster processing, may not reach optimum

**Convergence Threshold:** Default 1.0

- Lower values: More precise convergence, slower
- Higher values: Faster termination, less precise

**Initialization Method:** K-Means++

- Random: Faster, less reliable
- K-Means++: Slower initialization, better results

## Performance Characteristics

### Time Complexity: O(n × k × i × d)

- n = number of pixels
- k = number of clusters (colors)
- i = number of iterations
- d = color dimensions (3 for RGB)

### Space Complexity: O(n + k)

- Store cluster assignment for each pixel: O(n)
- Store centroid colors: O(k)

### Benchmarks

Typical processing times:

| Image Size | Colors | Grid Size | Iterations | Time  |
| ---------- | ------ | --------- | ---------- | ----- |
| 512×512    | 16     | 32×32     | 15         | 0.12s |
| 1024×1024  | 16     | 48×48     | 20         | 0.35s |
| 1024×1024  | 32     | 48×48     | 25         | 0.65s |
| 2048×2048  | 16     | 64×64     | 18         | 1.2s  |

## Best Use Cases

### Portrait Photography

Excellent for skin tones and facial features:

```bash
pixel-art-rust -i portrait.jpg -o pixel_portrait.png -w 64 -h 80 \
  --algorithm kmeans --colors 24
```

### Artistic Illustrations

Preserves color relationships in artwork:

```bash
pixel-art-rust -i digital_art.png -o pixel_art.png -w 96 -h 96 \
  --algorithm kmeans --colors 32
```

### High-Quality Conversions

When quality is more important than speed:

```bash
pixel-art-rust -i photo.jpg -o quality.png -w 80 -h 60 \
  --algorithm kmeans --colors 40
```

### Final Production Work

For published artwork or professional projects:

```bash
pixel-art-rust -i masterpiece.jpg -o final.png -w 128 -h 96 \
  --algorithm kmeans --colors 64
```

## Color Space Considerations

### RGB Color Space

**Advantages:**

- Simple implementation
- Fast computation
- Direct pixel representation

**Disadvantages:**

- Not perceptually uniform
- Poor color distance calculation
- Can produce unnatural groupings

### LAB Color Space (Recommended)

**Advantages:**

- Perceptually uniform
- Better color distance calculation
- More natural color groupings

**Disadvantages:**

- Requires color space conversion
- Slightly slower computation
- More complex implementation

### Distance Metrics

**Euclidean Distance (RGB):**

```rust
fn rgb_distance(a: &RGB, b: &RGB) -> f32 {
    let dr = (a.r as f32 - b.r as f32);
    let dg = (a.g as f32 - b.g as f32);
    let db = (a.b as f32 - b.b as f32);
    (dr*dr + dg*dg + db*db).sqrt()
}
```

**Delta E 2000 (LAB):**

```rust
fn delta_e_2000(a: &LAB, b: &LAB) -> f32 {
    // Complex perceptual color difference calculation
    // Accounts for human vision characteristics
    // More accurate but computationally expensive
}
```

## Optimization Techniques

### Parallel Processing

K-Means is highly parallelizable:

```rust
// Parallel assignment step
let assignments: Vec<usize> = colors.par_iter()
    .map(|color| find_nearest_centroid(color, &centroids))
    .collect();

// Parallel centroid update
let new_centroids: Vec<Color> = (0..k).into_par_iter()
    .map(|cluster| calculate_centroid(colors, &assignments, cluster))
    .collect();
```

### Early Termination

Stop when improvement is minimal:

```rust
if improvement_ratio < 0.01 {
    break; // Minimal improvement, stop iterating
}
```

### Smart Initialization

Use domain knowledge for better starting points:

```rust
// For photos: start with common photo colors
let photo_palette = [skin_tone, sky_blue, grass_green, shadow_gray];

// For artwork: extract dominant colors first
let dominant_colors = extract_dominant_colors(image, k);
```

## Common Issues and Solutions

### Slow Convergence

**Problem**: Algorithm takes too many iterations

**Solutions:**

- Use K-Means++ initialization
- Reduce convergence threshold
- Set maximum iteration limit
- Use fewer colors initially

```bash
# Start with fewer colors for speed
pixel-art-rust -i large.jpg -o test.png -w 32 -h 32 \
  --algorithm kmeans --colors 12
```

### Poor Color Selection

**Problem**: Important colors are missing or unnatural

**Solutions:**

- Try different runs (due to randomness)
- Adjust number of colors
- Preprocess image to enhance important colors

```bash
# Enhance contrast before processing
convert input.jpg -contrast-stretch 5%x5% enhanced.jpg
pixel-art-rust -i enhanced.jpg -o output.png -w 48 -h 48 \
  --algorithm kmeans --colors 16
```

### Memory Issues

**Problem**: Out of memory with large images

**Solutions:**

- Reduce grid resolution
- Use image subsampling
- Process image in tiles

```bash
# Reduce grid size for large images
pixel-art-rust -i huge_image.jpg -o output.png -w 64 -h 48 \
  --algorithm kmeans --colors 16
```

## Comparison with Other Algorithms

### vs. Average Color

- **Quality**: K-Means produces significantly better results
- **Speed**: ~20-50x slower than average
- **Consistency**: K-Means can vary, average is deterministic
- **Use case**: K-Means for quality, average for speed

### vs. Median Cut

- **Quality**: K-Means often produces superior color palettes
- **Speed**: ~3-5x slower than median cut
- **Determinism**: Median cut is consistent, K-Means varies
- **Use case**: K-Means for best quality, median cut for consistency

### vs. Quadtree

- **Focus**: K-Means optimizes colors, quadtree optimizes spatial detail
- **Quality**: Different strengths - color vs. spatial
- **Speed**: Similar performance for comparable quality
- **Use case**: K-Means for color-rich images, quadtree for detailed images

## Advanced Techniques

### Hierarchical K-Means

Improve quality with multi-level clustering:

1. **Coarse clustering**: Use K-Means with more colors than target
2. **Merge step**: Combine similar colors to reach target count
3. **Refinement**: Final K-Means pass with target color count

### Constrained K-Means

Force certain colors to be included:

```rust
// Always include pure black and white
let fixed_colors = [Color::BLACK, Color::WHITE];
let variable_colors = kmeans(colors, k - fixed_colors.len());
let final_palette = [fixed_colors, variable_colors].concat();
```

### Weighted K-Means

Give more importance to certain pixels:

```rust
// Weight pixels by their visual importance
let weights = calculate_saliency_map(image);
let weighted_result = weighted_kmeans(colors, weights, k);
```

## Example Workflows

### Professional Photography

```bash
# High-quality portrait conversion
pixel-art-rust -i portrait.jpg -o professional.png -w 96 -h 128 \
  --algorithm kmeans --colors 32
```

### Game Asset Creation

```bash
# Consistent palette for game sprites
pixel-art-rust -i character.png -o sprite.png -w 32 -h 32 \
  --algorithm kmeans --colors 16
```

### Artistic Projects

```bash
# Fine art conversion with many colors
pixel-art-rust -i painting.jpg -o pixel_painting.png -w 120 -h 90 \
  --algorithm kmeans --colors 48
```

### Batch Processing with Quality

```bash
for img in gallery/*.jpg; do
    name=$(basename "$img" .jpg)
    pixel-art-rust -i "$img" -o "output/${name}_quality.png" -w 64 -h 64 \
      --algorithm kmeans --colors 24
done
```

## See Also

- [Median Cut Algorithm](/algorithms/median-cut) - Faster alternative with good quality
- [Average Color Algorithm](/algorithms/average-color) - Much faster for previews
- [Quadtree Algorithm](/algorithms/quadtree) - Spatial detail preservation
- [Algorithm Overview](/algorithms/overview) - Complete algorithm comparison
