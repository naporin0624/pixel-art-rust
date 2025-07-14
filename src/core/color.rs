use image::Rgba;
use lazy_static::lazy_static;
use palette::{FromColor, Lab, Srgb};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub trait ColorExtractor: Send + Sync {
    fn extract_color(&self, pixels: &[Rgba<u8>]) -> Rgba<u8>;
}

pub struct AverageColorExtractor;

impl ColorExtractor for AverageColorExtractor {
    fn extract_color(&self, pixels: &[Rgba<u8>]) -> Rgba<u8> {
        if pixels.is_empty() {
            return Rgba([0, 0, 0, 255]);
        }

        let mut total_r = 0u32;
        let mut total_g = 0u32;
        let mut total_b = 0u32;

        for pixel in pixels {
            total_r += pixel.0[0] as u32;
            total_g += pixel.0[1] as u32;
            total_b += pixel.0[2] as u32;
        }

        let count = pixels.len() as u32;
        Rgba([
            (total_r / count) as u8,
            (total_g / count) as u8,
            (total_b / count) as u8,
            255,
        ])
    }
}

pub struct MedianCutExtractor {
    pub max_colors: u32,
}

impl ColorExtractor for MedianCutExtractor {
    fn extract_color(&self, pixels: &[Rgba<u8>]) -> Rgba<u8> {
        if pixels.is_empty() {
            return Rgba([0, 0, 0, 255]);
        }

        if pixels.len() == 1 {
            return pixels[0];
        }

        // Simple implementation: return the median color
        let mut sorted_pixels = pixels.to_vec();
        sorted_pixels.sort_by(|a, b| {
            let a_lum = (a.0[0] as u32 + a.0[1] as u32 + a.0[2] as u32) / 3;
            let b_lum = (b.0[0] as u32 + b.0[1] as u32 + b.0[2] as u32) / 3;
            a_lum.cmp(&b_lum)
        });

        sorted_pixels[sorted_pixels.len() / 2]
    }
}

pub struct KMeansExtractor {
    pub k: u32,
    pub max_iterations: u32,
}

impl ColorExtractor for KMeansExtractor {
    fn extract_color(&self, pixels: &[Rgba<u8>]) -> Rgba<u8> {
        if pixels.is_empty() {
            return Rgba([0, 0, 0, 255]);
        }

        if pixels.len() == 1 {
            return pixels[0];
        }

        // Simple implementation: return the most common color
        let mut color_counts = std::collections::HashMap::new();
        for pixel in pixels {
            *color_counts.entry(*pixel).or_insert(0) += 1;
        }

        color_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(color, _)| color)
            .unwrap_or(pixels[0])
    }
}

pub fn color_distance_lab(color1: &Rgba<u8>, color2: &Rgba<u8>) -> f64 {
    let rgb1 = Srgb::new(
        color1.0[0] as f32 / 255.0,
        color1.0[1] as f32 / 255.0,
        color1.0[2] as f32 / 255.0,
    );
    let rgb2 = Srgb::new(
        color2.0[0] as f32 / 255.0,
        color2.0[1] as f32 / 255.0,
        color2.0[2] as f32 / 255.0,
    );

    let lab1 = Lab::from_color(rgb1);
    let lab2 = Lab::from_color(rgb2);

    let dl = lab1.l - lab2.l;
    let da = lab1.a - lab2.a;
    let db = lab1.b - lab2.b;

    (dl * dl + da * da + db * db).sqrt() as f64
}

pub fn calculate_color_variance(pixels: &[Rgba<u8>]) -> (Rgba<u8>, f64) {
    if pixels.is_empty() {
        return (Rgba([0, 0, 0, 255]), 0.0);
    }

    let extractor = AverageColorExtractor;
    let mean = extractor.extract_color(pixels);

    let mut variance = 0.0;
    for pixel in pixels {
        let distance = color_distance_lab(&mean, pixel);
        variance += distance * distance;
    }

    variance /= pixels.len() as f64;
    (mean, variance)
}

pub struct SimdAverageColorExtractor;

impl ColorExtractor for SimdAverageColorExtractor {
    fn extract_color(&self, pixels: &[Rgba<u8>]) -> Rgba<u8> {
        if pixels.is_empty() {
            return Rgba([0, 0, 0, 255]);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { extract_color_avx2(pixels) };
            }
        }

        // Fallback to regular implementation
        AverageColorExtractor.extract_color(pixels)
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn extract_color_avx2(pixels: &[Rgba<u8>]) -> Rgba<u8> {
    unsafe {
        let mut sum_r = _mm256_setzero_si256();
        let mut sum_g = _mm256_setzero_si256();
        let mut sum_b = _mm256_setzero_si256();

        let chunks = pixels.chunks_exact(8);
        let remainder = chunks.remainder();

        for chunk in chunks {
            // Load 8 RGBA pixels into registers
            let mut r_vals = [0u16; 16];
            let mut g_vals = [0u16; 16];
            let mut b_vals = [0u16; 16];

            for (i, pixel) in chunk.iter().enumerate() {
                r_vals[i] = pixel.0[0] as u16;
                g_vals[i] = pixel.0[1] as u16;
                b_vals[i] = pixel.0[2] as u16;
            }

            let r = _mm256_loadu_si256(r_vals.as_ptr() as *const __m256i);
            let g = _mm256_loadu_si256(g_vals.as_ptr() as *const __m256i);
            let b = _mm256_loadu_si256(b_vals.as_ptr() as *const __m256i);

            sum_r = _mm256_add_epi16(sum_r, r);
            sum_g = _mm256_add_epi16(sum_g, g);
            sum_b = _mm256_add_epi16(sum_b, b);
        }

        // Horizontal sum
        let mut total_r = horizontal_sum_avx2(sum_r) as u32;
        let mut total_g = horizontal_sum_avx2(sum_g) as u32;
        let mut total_b = horizontal_sum_avx2(sum_b) as u32;

        // Handle remainder
        for pixel in remainder {
            total_r += pixel.0[0] as u32;
            total_g += pixel.0[1] as u32;
            total_b += pixel.0[2] as u32;
        }

        let count = pixels.len() as u32;
        Rgba([
            (total_r / count) as u8,
            (total_g / count) as u8,
            (total_b / count) as u8,
            255,
        ])
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn horizontal_sum_avx2(v: __m256i) -> u16 {
    let v128 = _mm256_extracti128_si256(v, 1);
    let v64 = _mm_add_epi16(_mm256_castsi256_si128(v), v128);
    let v32 = _mm_hadd_epi16(v64, v64);
    let v16 = _mm_hadd_epi16(v32, v32);
    let v8 = _mm_hadd_epi16(v16, v16);
    _mm_extract_epi16(v8, 0) as u16
}

lazy_static! {
    static ref GAMMA_LUT: [f32; 256] = {
        let mut lut = [0.0; 256];
        for (i, item) in lut.iter_mut().enumerate() {
            let v = i as f32 / 255.0;
            *item = if v > 0.04045 {
                ((v + 0.055) / 1.055).powf(2.4)
            } else {
                v / 12.92
            };
        }
        lut
    };

    static ref LAB_F_LUT: [f32; 4096] = {
        let mut lut = [0.0; 4096];
        for (i, item) in lut.iter_mut().enumerate() {
            let t = i as f32 / 4095.0 * 2.0; // scale to [0, 2]
            *item = if t > 0.008856 {
                t.powf(1.0 / 3.0)
            } else {
                7.787 * t + 16.0 / 116.0
            };
        }
        lut
    };
}

pub fn color_distance_lab_fast(color1: &Rgba<u8>, color2: &Rgba<u8>) -> f64 {
    // Use LUTs for gamma correction
    let r1 = GAMMA_LUT[color1.0[0] as usize];
    let g1 = GAMMA_LUT[color1.0[1] as usize];
    let b1 = GAMMA_LUT[color1.0[2] as usize];

    let r2 = GAMMA_LUT[color2.0[0] as usize];
    let g2 = GAMMA_LUT[color2.0[1] as usize];
    let b2 = GAMMA_LUT[color2.0[2] as usize];

    // Matrix multiplication for XYZ (fused constants)
    let x1 = 0.4124564 * r1 + 0.3575761 * g1 + 0.1804375 * b1;
    let y1 = 0.2126729 * r1 + 0.7151522 * g1 + 0.0721750 * b1;
    let z1 = 0.0193339 * r1 + 0.119_192 * g1 + 0.9503041 * b1;

    let x2 = 0.4124564 * r2 + 0.3575761 * g2 + 0.1804375 * b2;
    let y2 = 0.2126729 * r2 + 0.7151522 * g2 + 0.0721750 * b2;
    let z2 = 0.0193339 * r2 + 0.119_192 * g2 + 0.9503041 * b2;

    // Use LUT for f(t) function
    let fx1 = LAB_F_LUT[((x1 / 0.95047).clamp(0.0, 2.0) * 2047.5) as usize];
    let fy1 = LAB_F_LUT[(y1.clamp(0.0, 2.0) * 2047.5) as usize];
    let fz1 = LAB_F_LUT[((z1 / 1.08883).clamp(0.0, 2.0) * 2047.5) as usize];

    let fx2 = LAB_F_LUT[((x2 / 0.95047).clamp(0.0, 2.0) * 2047.5) as usize];
    let fy2 = LAB_F_LUT[(y2.clamp(0.0, 2.0) * 2047.5) as usize];
    let fz2 = LAB_F_LUT[((z2 / 1.08883).clamp(0.0, 2.0) * 2047.5) as usize];

    // Compute L*a*b*
    let l1 = 116.0 * fy1 - 16.0;
    let a1 = 500.0 * (fx1 - fy1);
    let b1_lab = 200.0 * (fy1 - fz1);

    let l2 = 116.0 * fy2 - 16.0;
    let a2 = 500.0 * (fx2 - fy2);
    let b2_lab = 200.0 * (fy2 - fz2);

    // Euclidean distance in L*a*b* space
    let dl = l1 - l2;
    let da = a1 - a2;
    let db = b1_lab - b2_lab;

    (dl * dl + da * da + db * db).sqrt() as f64
}

pub struct SoAAverageColorExtractor;

impl ColorExtractor for SoAAverageColorExtractor {
    fn extract_color(&self, pixels: &[Rgba<u8>]) -> Rgba<u8> {
        if pixels.is_empty() {
            return Rgba([0, 0, 0, 255]);
        }

        // Separate RGBA channels for better cache locality
        let len = pixels.len();
        let mut sum_r = 0u32;
        let mut sum_g = 0u32;
        let mut sum_b = 0u32;

        // Process in chunks for better cache utilization
        const CHUNK_SIZE: usize = 64;
        for chunk in pixels.chunks(CHUNK_SIZE) {
            for pixel in chunk {
                sum_r += pixel.0[0] as u32;
                sum_g += pixel.0[1] as u32;
                sum_b += pixel.0[2] as u32;
            }
        }

        let count = len as u32;
        Rgba([
            (sum_r / count) as u8,
            (sum_g / count) as u8,
            (sum_b / count) as u8,
            255,
        ])
    }
}

pub fn hierarchical_color_clustering(
    pixels: &[Rgba<u8>],
    num_clusters: usize,
) -> Vec<Vec<Rgba<u8>>> {
    if pixels.is_empty() || num_clusters == 0 {
        return vec![];
    }

    if num_clusters >= pixels.len() {
        return pixels.iter().map(|&p| vec![p]).collect();
    }

    // Simple implementation: divide pixels into clusters based on color similarity
    let mut clusters: Vec<Vec<Rgba<u8>>> = Vec::new();
    let mut remaining_pixels = pixels.to_vec();

    for _ in 0..num_clusters {
        if remaining_pixels.is_empty() {
            break;
        }

        let mut cluster = vec![remaining_pixels.remove(0)];
        let cluster_center = cluster[0];

        remaining_pixels.retain(|&pixel| {
            let distance = color_distance_lab_fast(&cluster_center, &pixel);
            if distance < 40.0 {
                cluster.push(pixel);
                false
            } else {
                true
            }
        });

        clusters.push(cluster);
    }

    // Add remaining pixels to the nearest cluster
    for pixel in remaining_pixels {
        let mut min_distance = f64::MAX;
        let mut best_cluster = 0;

        for (i, cluster) in clusters.iter().enumerate() {
            if !cluster.is_empty() {
                let distance = color_distance_lab_fast(&cluster[0], &pixel);
                if distance < min_distance {
                    min_distance = distance;
                    best_cluster = i;
                }
            }
        }

        clusters[best_cluster].push(pixel);
    }

    clusters
}
