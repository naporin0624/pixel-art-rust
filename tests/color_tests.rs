use image::Rgba;
use pixel_art_rust::core::color::*;

#[test]
fn test_average_color_extraction() {
    let extractor = AverageColorExtractor;

    // Test with uniform colors
    let pixels = vec![
        Rgba([255, 0, 0, 255]),
        Rgba([255, 0, 0, 255]),
        Rgba([255, 0, 0, 255]),
    ];
    let result = extractor.extract_color(&pixels);
    assert_eq!(result, Rgba([255, 0, 0, 255]));

    // Test with mixed colors
    let pixels = vec![
        Rgba([255, 0, 0, 255]),
        Rgba([0, 255, 0, 255]),
        Rgba([0, 0, 255, 255]),
    ];
    let result = extractor.extract_color(&pixels);
    assert_eq!(result, Rgba([85, 85, 85, 255]));

    // Test with empty pixels
    let pixels = vec![];
    let result = extractor.extract_color(&pixels);
    assert_eq!(result, Rgba([0, 0, 0, 255]));
}

#[test]
fn test_median_cut_algorithm() {
    let extractor = MedianCutExtractor { max_colors: 2 };

    // Test with distinct colors
    let pixels = vec![
        Rgba([255, 0, 0, 255]),
        Rgba([0, 255, 0, 255]),
        Rgba([0, 0, 255, 255]),
        Rgba([255, 255, 0, 255]),
    ];

    let result = extractor.extract_color(&pixels);
    // Should return one of the representative colors
    assert!(result.0[3] == 255); // Alpha should be 255

    // Test with single color
    let pixels = vec![Rgba([128, 128, 128, 255])];
    let result = extractor.extract_color(&pixels);
    assert_eq!(result, Rgba([128, 128, 128, 255]));
}

#[test]
fn test_kmeans_clustering() {
    let extractor = KMeansExtractor {
        k: 3,
        max_iterations: 10,
    };

    // Test with clustered colors
    let pixels = vec![
        Rgba([255, 0, 0, 255]),
        Rgba([250, 5, 5, 255]),
        Rgba([0, 255, 0, 255]),
        Rgba([5, 250, 5, 255]),
        Rgba([0, 0, 255, 255]),
        Rgba([5, 5, 250, 255]),
    ];

    let result = extractor.extract_color(&pixels);
    // Should return a representative color
    assert!(result.0[3] == 255); // Alpha should be 255
}

#[test]
fn test_color_distance_in_lab_space() {
    // Test identical colors
    let color1 = Rgba([255, 0, 0, 255]);
    let color2 = Rgba([255, 0, 0, 255]);
    let distance = color_distance_lab(&color1, &color2);
    assert!(distance < 0.01); // Should be very close to 0

    // Test different colors
    let color1 = Rgba([255, 0, 0, 255]);
    let color2 = Rgba([0, 255, 0, 255]);
    let distance = color_distance_lab(&color1, &color2);
    assert!(distance > 10.0); // Should be significantly different

    // Test black and white
    let color1 = Rgba([0, 0, 0, 255]);
    let color2 = Rgba([255, 255, 255, 255]);
    let distance = color_distance_lab(&color1, &color2);
    assert!(distance > 50.0); // Should be very different
}

#[test]
fn test_quadtree_color_variance() {
    // Test with uniform colors (low variance)
    let pixels = vec![
        Rgba([100, 100, 100, 255]),
        Rgba([101, 101, 101, 255]),
        Rgba([99, 99, 99, 255]),
    ];
    let (mean, variance) = calculate_color_variance(&pixels);
    assert!(variance < 5.0); // Should be low variance
    assert!(mean.0[0] >= 99 && mean.0[0] <= 101); // Mean should be around 100

    // Test with diverse colors (high variance)
    let pixels = vec![
        Rgba([0, 0, 0, 255]),
        Rgba([255, 255, 255, 255]),
        Rgba([255, 0, 0, 255]),
        Rgba([0, 255, 0, 255]),
    ];
    let (_mean, variance) = calculate_color_variance(&pixels);
    assert!(variance > 50.0); // Should be high variance
}

#[test]
fn test_hierarchical_clustering() {
    // Test with colors that should cluster into groups
    let pixels = vec![
        // Red cluster
        Rgba([255, 0, 0, 255]),
        Rgba([250, 10, 10, 255]),
        Rgba([245, 5, 5, 255]),
        // Blue cluster
        Rgba([0, 0, 255, 255]),
        Rgba([10, 10, 250, 255]),
        Rgba([5, 5, 245, 255]),
    ];

    let clusters = hierarchical_color_clustering(&pixels, 2);
    assert_eq!(clusters.len(), 2);

    // Each cluster should have similar colors
    for cluster in &clusters {
        assert!(cluster.len() >= 2);
        let first_color = &cluster[0];
        for color in cluster {
            let distance = color_distance_lab(first_color, color);
            assert!(distance < 30.0); // Colors in same cluster should be similar
        }
    }
}
