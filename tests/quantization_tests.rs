use image::Rgba;
use pixel_art_rust::core::quantization::*;

#[test]
fn test_median_cut_quantizer_basic() {
    let pixels = vec![
        Rgba([255, 0, 0, 255]),
        Rgba([0, 255, 0, 255]),
        Rgba([0, 0, 255, 255]),
        Rgba([255, 255, 0, 255]),
        Rgba([255, 0, 255, 255]),
        Rgba([0, 255, 255, 255]),
    ];

    let palette = MedianCutQuantizer::quantize(&pixels, 3);

    assert_eq!(palette.len(), 3);
    // All colors should have alpha 255
    for color in &palette {
        assert_eq!(color.0[3], 255);
    }
}

#[test]
fn test_median_cut_quantizer_single_color() {
    let pixels = vec![
        Rgba([128, 128, 128, 255]),
        Rgba([128, 128, 128, 255]),
        Rgba([128, 128, 128, 255]),
    ];

    let palette = MedianCutQuantizer::quantize(&pixels, 3);

    assert_eq!(palette.len(), 1);
    assert_eq!(palette[0], Rgba([128, 128, 128, 255]));
}

#[test]
fn test_median_cut_quantizer_empty() {
    let pixels = vec![];
    let palette = MedianCutQuantizer::quantize(&pixels, 3);

    assert!(palette.is_empty());
}

#[test]
fn test_median_cut_quantizer_more_colors_than_pixels() {
    let pixels = vec![Rgba([255, 0, 0, 255]), Rgba([0, 255, 0, 255])];

    let palette = MedianCutQuantizer::quantize(&pixels, 5);

    assert_eq!(palette.len(), 2);
    assert!(palette.contains(&Rgba([255, 0, 0, 255])));
    assert!(palette.contains(&Rgba([0, 255, 0, 255])));
}

#[test]
fn test_color_bucket_creation() {
    let pixels = vec![
        Rgba([255, 0, 0, 255]),
        Rgba([0, 255, 0, 255]),
        Rgba([0, 0, 255, 255]),
    ];

    let bucket = ColorBucket::new(pixels.clone());

    assert_eq!(bucket.pixels.len(), 3);
    assert_eq!(bucket.pixels, pixels);
}

#[test]
fn test_color_bucket_range_calculation() {
    let pixels = vec![
        Rgba([100, 50, 200, 255]),
        Rgba([150, 100, 100, 255]),
        Rgba([200, 150, 50, 255]),
    ];

    let bucket = ColorBucket::new(pixels);
    let ranges = bucket.get_color_ranges();

    // Red range: 200 - 100 = 100
    // Green range: 150 - 50 = 100
    // Blue range: 200 - 50 = 150
    assert_eq!(ranges.0, 100); // Red range
    assert_eq!(ranges.1, 100); // Green range
    assert_eq!(ranges.2, 150); // Blue range
}

#[test]
fn test_find_largest_axis() {
    let pixels = vec![Rgba([0, 50, 100, 255]), Rgba([255, 60, 110, 255])];

    let bucket = ColorBucket::new(pixels);
    let axis = MedianCutQuantizer::find_largest_axis(&bucket);

    // Red has range 255, green has range 10, blue has range 10
    // So red should be the largest axis
    assert_eq!(axis, Axis::Red);
}

#[test]
fn test_bucket_splitting() {
    let pixels = vec![
        Rgba([50, 100, 150, 255]),
        Rgba([100, 100, 150, 255]),
        Rgba([150, 100, 150, 255]),
        Rgba([200, 100, 150, 255]),
    ];

    let bucket = ColorBucket::new(pixels);
    let (left, right) = MedianCutQuantizer::split_bucket(bucket, Axis::Red);

    // Should split roughly in the middle
    assert_eq!(left.pixels.len(), 2);
    assert_eq!(right.pixels.len(), 2);

    // Left bucket should have smaller red values
    for pixel in &left.pixels {
        assert!(pixel.0[0] <= 100);
    }

    // Right bucket should have larger red values
    for pixel in &right.pixels {
        assert!(pixel.0[0] >= 100);
    }
}

#[test]
fn test_representative_color_extraction() {
    let pixels = vec![
        Rgba([100, 150, 200, 255]),
        Rgba([110, 160, 210, 255]),
        Rgba([90, 140, 190, 255]),
    ];

    let bucket = ColorBucket::new(pixels);
    let representative = bucket.get_representative_color();

    // Should be close to the average
    assert!(representative.0[0] >= 95 && representative.0[0] <= 105);
    assert!(representative.0[1] >= 145 && representative.0[1] <= 155);
    assert!(representative.0[2] >= 195 && representative.0[2] <= 205);
    assert_eq!(representative.0[3], 255);
}
