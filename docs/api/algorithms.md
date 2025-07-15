# Algorithm API Reference

Technical documentation for the color quantization algorithms implemented in Pixel Art Rust.

## Overview

Pixel Art Rust implements four main algorithms for color quantization and pixel art generation. Each algorithm has different trade-offs between speed, quality, and memory usage.

## Algorithm Trait

All algorithms implement the core `ColorQuantizer` trait:

```rust
pub trait ColorQuantizer {
    fn quantize(&self, colors: &[Rgb<u8>]) -> Result<Rgb<u8>, QuantizeError>;
    fn quantize_batch(&self, regions: &[Vec<Rgb<u8>>]) -> Result<Vec<Rgb<u8>>, QuantizeError>;
}
```

## Average Color Algorithm

### Implementation

```rust
pub struct AverageColorQuantizer;

impl ColorQuantizer for AverageColorQuantizer {
    fn quantize(&self, colors: &[Rgb<u8>]) -> Result<Rgb<u8>, QuantizeError> {
        if colors.is_empty() {
            return Err(QuantizeError::EmptyInput);
        }

        let sum = colors.iter().fold([0u64; 3], |mut acc, &Rgb([r, g, b])| {
            acc[0] += r as u64;
            acc[1] += g as u64;
            acc[2] += b as u64;
            acc
        });

        let count = colors.len() as u64;
        Ok(Rgb([
            (sum[0] / count) as u8,
            (sum[1] / count) as u8,
            (sum[2] / count) as u8,
        ]))
    }
}
```

### Characteristics

- **Time Complexity:** O(n) where n is the number of pixels
- **Space Complexity:** O(1)
- **Best For:** Fast previews, uniform images, batch processing
- **Limitations:** May produce bland colors in diverse regions

### Configuration

```rust
use pixel_art_rust::algorithms::AverageColorQuantizer;

let quantizer = AverageColorQuantizer;
// No additional configuration required
```

## Median Cut Algorithm

### Implementation Overview

The median cut algorithm recursively divides the color space to create a balanced color palette.

```rust
pub struct MedianCutQuantizer {
    color_count: usize,
}

impl MedianCutQuantizer {
    pub fn new(color_count: usize) -> Self {
        Self { color_count }
    }

    fn median_cut_recursive(
        &self,
        colors: &mut [LabColor],
        depth: usize
    ) -> Vec<LabColor> {
        if colors.len() <= 1 || depth == 0 {
            return vec![self.average_color(colors)];
        }

        let axis = self.find_longest_axis(colors);
        colors.sort_by(|a, b| self.compare_along_axis(a, b, axis));

        let mid = colors.len() / 2;
        let (left, right) = colors.split_at_mut(mid);

        let mut result = self.median_cut_recursive(left, depth - 1);
        result.extend(self.median_cut_recursive(right, depth - 1));
        result
    }
}
```

### Color Space Operations

```rust
impl MedianCutQuantizer {
    fn find_longest_axis(&self, colors: &[LabColor]) -> ColorAxis {
        let mut ranges = [0.0f32; 3];

        for axis in 0..3 {
            let values: Vec<f32> = colors.iter()
                .map(|c| c.component(axis))
                .collect();
            ranges[axis] = values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
                - values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        }

        ranges.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| ColorAxis::from(i))
            .unwrap()
    }
}
```

### Characteristics

- **Time Complexity:** O(n log n × c) where c is color count
- **Space Complexity:** O(n + c)
- **Best For:** Balanced quality and performance
- **Advantages:** Good color distribution, handles gradients well

### Configuration

```rust
use pixel_art_rust::algorithms::MedianCutQuantizer;

let quantizer = MedianCutQuantizer::new(16); // 16 colors
```

### Advanced Usage

```rust
use pixel_art_rust::algorithms::{MedianCutQuantizer, MedianCutConfig};

let config = MedianCutConfig {
    color_count: 32,
    color_space: ColorSpace::LAB, // More perceptually uniform
    split_threshold: 1.0,         // Minimum variance for splitting
};

let quantizer = MedianCutQuantizer::with_config(config);
```

## K-Means Algorithm

### Implementation Overview

K-means clustering for optimal color palette generation.

```rust
pub struct KMeansQuantizer {
    k: usize,
    max_iterations: usize,
    convergence_threshold: f32,
}

impl KMeansQuantizer {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            max_iterations: 100,
            convergence_threshold: 1.0,
        }
    }

    fn cluster(&self, colors: &[LabColor]) -> Result<Vec<LabColor>, QuantizeError> {
        let mut centroids = self.initialize_centroids(colors)?;

        for iteration in 0..self.max_iterations {
            let assignments = self.assign_clusters(colors, &centroids);
            let new_centroids = self.update_centroids(colors, &assignments)?;

            if self.has_converged(&centroids, &new_centroids) {
                break;
            }

            centroids = new_centroids;
        }

        Ok(centroids)
    }
}
```

### Centroid Initialization

```rust
impl KMeansQuantizer {
    fn initialize_centroids(&self, colors: &[LabColor]) -> Result<Vec<LabColor>, QuantizeError> {
        if colors.len() < self.k {
            return Err(QuantizeError::InsufficientColors);
        }

        // K-means++ initialization for better convergence
        let mut centroids = Vec::with_capacity(self.k);
        let mut rng = thread_rng();

        // First centroid: random selection
        centroids.push(colors[rng.gen_range(0..colors.len())]);

        // Subsequent centroids: weighted by distance to nearest existing centroid
        for _ in 1..self.k {
            let distances: Vec<f32> = colors.iter()
                .map(|color| self.min_distance_to_centroids(color, &centroids))
                .collect();

            let total_distance: f32 = distances.iter().sum();
            let mut threshold = rng.gen::<f32>() * total_distance;

            for (i, &distance) in distances.iter().enumerate() {
                threshold -= distance;
                if threshold <= 0.0 {
                    centroids.push(colors[i]);
                    break;
                }
            }
        }

        Ok(centroids)
    }
}
```

### Clustering Operations

```rust
impl KMeansQuantizer {
    fn assign_clusters(&self, colors: &[LabColor], centroids: &[LabColor]) -> Vec<usize> {
        colors.par_iter() // Parallel processing with Rayon
            .map(|color| {
                centroids.iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        self.color_distance(color, a)
                            .partial_cmp(&self.color_distance(color, b))
                            .unwrap()
                    })
                    .map(|(i, _)| i)
                    .unwrap()
            })
            .collect()
    }

    fn update_centroids(
        &self,
        colors: &[LabColor],
        assignments: &[usize]
    ) -> Result<Vec<LabColor>, QuantizeError> {
        let mut new_centroids = vec![LabColor::default(); self.k];
        let mut counts = vec![0usize; self.k];

        for (color, &cluster) in colors.iter().zip(assignments.iter()) {
            new_centroids[cluster] = new_centroids[cluster] + *color;
            counts[cluster] += 1;
        }

        for (centroid, count) in new_centroids.iter_mut().zip(counts.iter()) {
            if *count > 0 {
                *centroid = *centroid / (*count as f32);
            }
        }

        Ok(new_centroids)
    }
}
```

### Characteristics

- **Time Complexity:** O(n × k × i) where i is iterations
- **Space Complexity:** O(n + k)
- **Best For:** Highest quality color palettes
- **Advantages:** Optimal color clustering, excellent gradients

### Configuration

```rust
use pixel_art_rust::algorithms::{KMeansQuantizer, KMeansConfig};

let config = KMeansConfig {
    k: 24,
    max_iterations: 150,
    convergence_threshold: 0.5,
    initialization: InitializationMethod::KMeansPlusPlus,
};

let quantizer = KMeansQuantizer::with_config(config);
```

## Adaptive Quadtree Algorithm

### Implementation Overview

Dynamically subdivides image regions based on color variance.

```rust
pub struct QuadtreeNode {
    bounds: Rectangle,
    color: Option<LabColor>,
    children: Option<[Box<QuadtreeNode>; 4]>,
    variance: f32,
}

pub struct AdaptiveQuantizer {
    max_depth: usize,
    variance_threshold: f32,
}

impl AdaptiveQuantizer {
    fn build_quadtree(
        &self,
        image: &[LabColor],
        bounds: Rectangle,
        depth: usize,
    ) -> QuadtreeNode {
        let region_colors = self.extract_region_colors(image, bounds);
        let variance = self.calculate_variance(&region_colors);

        if depth >= self.max_depth || variance < self.variance_threshold {
            // Leaf node
            QuadtreeNode {
                bounds,
                color: Some(self.average_color(&region_colors)),
                children: None,
                variance,
            }
        } else {
            // Internal node - subdivide
            let subdivisions = self.subdivide_bounds(bounds);
            let children = subdivisions.map(|sub_bounds| {
                Box::new(self.build_quadtree(image, sub_bounds, depth + 1))
            });

            QuadtreeNode {
                bounds,
                color: None,
                children: Some(children),
                variance,
            }
        }
    }
}
```

### Variance Calculation

```rust
impl AdaptiveQuantizer {
    fn calculate_variance(&self, colors: &[LabColor]) -> f32 {
        if colors.len() <= 1 {
            return 0.0;
        }

        let mean = self.average_color(colors);
        let sum_squared_distances: f32 = colors.iter()
            .map(|color| self.color_distance(color, &mean).powi(2))
            .sum();

        sum_squared_distances / colors.len() as f32
    }

    fn color_distance(&self, a: &LabColor, b: &LabColor) -> f32 {
        // Delta E 2000 color difference formula
        delta_e_2000(a, b)
    }
}
```

### Rendering

```rust
impl AdaptiveQuantizer {
    fn render_quadtree(&self, node: &QuadtreeNode, output: &mut [Rgb<u8>], width: usize) {
        match &node.children {
            Some(children) => {
                // Render children recursively
                for child in children.iter() {
                    self.render_quadtree(child, output, width);
                }
            }
            None => {
                // Render leaf node
                if let Some(color) = node.color {
                    self.fill_region(output, node.bounds, color.to_rgb(), width);
                }
            }
        }
    }
}
```

### Characteristics

- **Time Complexity:** O(n × log(d)) where d is max depth
- **Space Complexity:** O(4^d) for quadtree structure
- **Best For:** Images with varying detail levels
- **Advantages:** Adaptive detail, efficient for mixed content

### Configuration

```rust
use pixel_art_rust::algorithms::{AdaptiveQuantizer, QuadtreeConfig};

let config = QuadtreeConfig {
    max_depth: 10,
    variance_threshold: 25.0,
    color_space: ColorSpace::LAB,
    min_region_size: 4, // Minimum pixels per region
};

let quantizer = AdaptiveQuantizer::with_config(config);
```

## Color Space Utilities

### LAB Color Space

```rust
#[derive(Clone, Copy, Debug)]
pub struct LabColor {
    pub l: f32, // Lightness (0-100)
    pub a: f32, // Green-Red axis (-128 to 127)
    pub b: f32, // Blue-Yellow axis (-128 to 127)
}

impl LabColor {
    pub fn from_rgb(rgb: Rgb<u8>) -> Self {
        let [r, g, b] = rgb.0;

        // Convert RGB to XYZ
        let rgb_normalized = [
            Self::gamma_correction(r as f32 / 255.0),
            Self::gamma_correction(g as f32 / 255.0),
            Self::gamma_correction(b as f32 / 255.0),
        ];

        // XYZ to LAB conversion
        // Implementation follows CIE standard
        // ...
    }

    pub fn to_rgb(self) -> Rgb<u8> {
        // LAB to XYZ to RGB conversion
        // ...
    }
}
```

### Delta E 2000

```rust
pub fn delta_e_2000(lab1: &LabColor, lab2: &LabColor) -> f32 {
    // CIE Delta E 2000 formula implementation
    // More perceptually uniform than Euclidean distance
    // ...
}
```

## Performance Optimizations

### SIMD Acceleration

```rust
#[cfg(target_feature = "avx2")]
fn average_colors_simd(colors: &[Rgb<u8>]) -> Rgb<u8> {
    use std::arch::x86_64::*;

    unsafe {
        // SIMD implementation for batch color averaging
        // Processes 8 colors at once with AVX2
        // ...
    }
}
```

### Memory Layout

```rust
// Structure of Arrays (SoA) for better cache performance
pub struct ColorBatch {
    r: Vec<u8>,
    g: Vec<u8>,
    b: Vec<u8>,
}

impl ColorBatch {
    pub fn from_rgb_slice(colors: &[Rgb<u8>]) -> Self {
        let mut batch = Self {
            r: Vec::with_capacity(colors.len()),
            g: Vec::with_capacity(colors.len()),
            b: Vec::with_capacity(colors.len()),
        };

        for rgb in colors {
            batch.r.push(rgb.0[0]);
            batch.g.push(rgb.0[1]);
            batch.b.push(rgb.0[2]);
        }

        batch
    }
}
```

## Benchmarking

### Performance Testing

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_algorithms(c: &mut Criterion) {
    let test_image = generate_test_image(1024, 1024);
    let colors: Vec<Rgb<u8>> = test_image.pixels().map(|p| Rgb(p.0)).collect();

    c.bench_function("average_quantizer", |b| {
        let quantizer = AverageColorQuantizer;
        b.iter(|| quantizer.quantize(black_box(&colors)))
    });

    c.bench_function("kmeans_quantizer", |b| {
        let quantizer = KMeansQuantizer::new(16);
        b.iter(|| quantizer.quantize(black_box(&colors)))
    });
}

criterion_group!(benches, benchmark_algorithms);
criterion_main!(benches);
```

## Algorithm Selection Guide

### Decision Matrix

```rust
pub fn select_algorithm(
    image_size: (u32, u32),
    target_quality: QualityLevel,
    time_budget: Duration,
) -> Box<dyn ColorQuantizer> {
    let pixel_count = image_size.0 * image_size.1;

    match (target_quality, pixel_count, time_budget) {
        (QualityLevel::Preview, _, _) => Box::new(AverageColorQuantizer),
        (QualityLevel::Balanced, _, t) if t < Duration::from_secs(1) => {
            Box::new(MedianCutQuantizer::new(16))
        },
        (QualityLevel::High, p, _) if p < 1_000_000 => {
            Box::new(KMeansQuantizer::new(32))
        },
        _ => Box::new(AdaptiveQuantizer::new(8, 30.0)),
    }
}
```

## See Also

- [Algorithm Details](/algorithms/overview) - Conceptual algorithm explanations
- [Core Library API](/api/core) - High-level library interface
- [CLI Reference](/api/cli) - Command-line usage examples
