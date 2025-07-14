use crate::core::color::AverageColorExtractor;
use crate::core::color::ColorExtractor;
use image::Rgba;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Axis {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
pub struct ColorBucket {
    pub pixels: Vec<Rgba<u8>>,
}

impl ColorBucket {
    pub fn new(pixels: Vec<Rgba<u8>>) -> Self {
        Self { pixels }
    }

    pub fn get_color_ranges(&self) -> (u8, u8, u8) {
        if self.pixels.is_empty() {
            return (0, 0, 0);
        }

        let mut min_r = 255u8;
        let mut max_r = 0u8;
        let mut min_g = 255u8;
        let mut max_g = 0u8;
        let mut min_b = 255u8;
        let mut max_b = 0u8;

        for pixel in &self.pixels {
            min_r = min_r.min(pixel.0[0]);
            max_r = max_r.max(pixel.0[0]);
            min_g = min_g.min(pixel.0[1]);
            max_g = max_g.max(pixel.0[1]);
            min_b = min_b.min(pixel.0[2]);
            max_b = max_b.max(pixel.0[2]);
        }

        (max_r - min_r, max_g - min_g, max_b - min_b)
    }

    pub fn get_representative_color(&self) -> Rgba<u8> {
        let extractor = AverageColorExtractor;
        extractor.extract_color(&self.pixels)
    }
}

pub struct MedianCutQuantizer {
    pub buckets: Vec<ColorBucket>,
}

impl MedianCutQuantizer {
    pub fn quantize(pixels: &[Rgba<u8>], target_colors: u32) -> Vec<Rgba<u8>> {
        if pixels.is_empty() {
            return vec![];
        }

        if target_colors == 0 {
            return vec![];
        }

        let mut buckets = vec![ColorBucket::new(pixels.to_vec())];

        // Split buckets until we have the target number of colors
        while buckets.len() < target_colors as usize {
            // Find the bucket with the largest color range
            let largest_bucket_idx = buckets
                .iter()
                .enumerate()
                .max_by_key(|(_, bucket)| {
                    let ranges = bucket.get_color_ranges();
                    ranges.0.max(ranges.1).max(ranges.2)
                })
                .map(|(idx, _)| idx);

            if let Some(idx) = largest_bucket_idx {
                let bucket = buckets.remove(idx);

                // If bucket has only one pixel, we can't split it
                if bucket.pixels.len() <= 1 {
                    buckets.push(bucket);
                    break;
                }

                // Check if all pixels are the same color (no color range)
                let ranges = bucket.get_color_ranges();
                if ranges.0 == 0 && ranges.1 == 0 && ranges.2 == 0 {
                    buckets.push(bucket);
                    break;
                }

                let largest_axis = Self::find_largest_axis(&bucket);
                let (left, right) = Self::split_bucket(bucket, largest_axis);

                buckets.push(left);
                buckets.push(right);
            } else {
                break;
            }
        }

        // Extract representative colors from each bucket
        buckets
            .into_iter()
            .map(|bucket| bucket.get_representative_color())
            .collect()
    }

    pub fn find_largest_axis(bucket: &ColorBucket) -> Axis {
        let ranges = bucket.get_color_ranges();

        if ranges.0 >= ranges.1 && ranges.0 >= ranges.2 {
            Axis::Red
        } else if ranges.1 >= ranges.2 {
            Axis::Green
        } else {
            Axis::Blue
        }
    }

    pub fn split_bucket(bucket: ColorBucket, axis: Axis) -> (ColorBucket, ColorBucket) {
        let mut pixels = bucket.pixels;

        // Sort pixels along the specified axis
        pixels.sort_by(|a, b| match axis {
            Axis::Red => a.0[0].cmp(&b.0[0]),
            Axis::Green => a.0[1].cmp(&b.0[1]),
            Axis::Blue => a.0[2].cmp(&b.0[2]),
        });

        let mid = pixels.len() / 2;
        let left_pixels = pixels[..mid].to_vec();
        let right_pixels = pixels[mid..].to_vec();

        (
            ColorBucket::new(left_pixels),
            ColorBucket::new(right_pixels),
        )
    }
}

// Fast color quantization with bit manipulation
#[inline(always)]
fn quantize_color_15bit(color: Rgba<u8>) -> u16 {
    let r = (color.0[0] >> 3) as u16;
    let g = (color.0[1] >> 3) as u16;
    let b = (color.0[2] >> 3) as u16;
    (r << 10) | (g << 5) | b
}

pub struct FastMedianCut {
    color_histogram: [u32; 32768], // 2^15 entries
}

impl FastMedianCut {
    pub fn new() -> Self {
        Self {
            color_histogram: [0; 32768],
        }
    }

    pub fn build_histogram(&mut self, pixels: &[Rgba<u8>]) {
        // Clear histogram
        self.color_histogram.fill(0);

        for pixel in pixels {
            let quantized = quantize_color_15bit(*pixel);
            self.color_histogram[quantized as usize] += 1;
        }
    }

    pub fn get_dominant_colors(&self, max_colors: usize) -> Vec<Rgba<u8>> {
        let mut colors_with_counts: Vec<(u16, u32)> = self
            .color_histogram
            .iter()
            .enumerate()
            .filter_map(|(i, &count)| {
                if count > 0 {
                    Some((i as u16, count))
                } else {
                    None
                }
            })
            .collect();

        // Sort by frequency (descending)
        colors_with_counts.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top colors and convert back to RGB
        colors_with_counts
            .into_iter()
            .take(max_colors)
            .map(|(quantized, _)| {
                let r = ((quantized >> 10) & 0x1f) << 3;
                let g = ((quantized >> 5) & 0x1f) << 3;
                let b = (quantized & 0x1f) << 3;
                Rgba([r as u8, g as u8, b as u8, 255])
            })
            .collect()
    }
}

impl Default for FastMedianCut {
    fn default() -> Self {
        Self::new()
    }
}

// Early termination for uniform regions
#[inline(always)]
pub fn is_uniform_region(pixels: &[Rgba<u8>], threshold: u8) -> Option<Rgba<u8>> {
    if pixels.is_empty() {
        return None;
    }

    let first = pixels[0];
    let mut all_same = true;

    // Vectorizable loop
    for pixel in pixels.iter().skip(1) {
        let dr = (first.0[0] as i16 - pixel.0[0] as i16).abs();
        let dg = (first.0[1] as i16 - pixel.0[1] as i16).abs();
        let db = (first.0[2] as i16 - pixel.0[2] as i16).abs();

        if dr > threshold as i16 || dg > threshold as i16 || db > threshold as i16 {
            all_same = false;
            break;
        }
    }

    if all_same { Some(first) } else { None }
}

// Color frequency analysis using HashMap for comparison
pub fn analyze_color_frequency(pixels: &[Rgba<u8>]) -> HashMap<Rgba<u8>, u32> {
    let mut frequency_map = HashMap::new();

    for pixel in pixels {
        *frequency_map.entry(*pixel).or_insert(0) += 1;
    }

    frequency_map
}
