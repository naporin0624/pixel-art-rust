use image::{Rgba, RgbaImage};
use pixel_art_rust::core::quadtree::*;

#[test]
fn test_quadtree_creation() {
    let mut image = RgbaImage::new(4, 4);

    // Fill with uniform color
    for pixel in image.pixels_mut() {
        *pixel = Rgba([128, 128, 128, 255]);
    }

    let tree = QuadTree::build(&image, 3, 50.0);

    // Root should exist and have the correct properties
    assert_eq!(tree.root.x, 0);
    assert_eq!(tree.root.y, 0);
    assert_eq!(tree.root.size, 4);
    assert_eq!(tree.root.mean_color, Rgba([128, 128, 128, 255]));
    assert!(tree.root.variance < 10.0); // Should be low variance
    assert!(tree.root.children.is_none()); // Should not split uniform colors
}

#[test]
fn test_quadtree_splitting() {
    let mut image = RgbaImage::new(4, 4);

    // Create a checkerboard pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = if (x + y) % 2 == 0 {
            Rgba([255, 255, 255, 255])
        } else {
            Rgba([0, 0, 0, 255])
        };
    }

    let tree = QuadTree::build(&image, 3, 10.0);

    // Should split due to high variance
    assert!(tree.root.children.is_some());
    assert!(tree.root.variance > 50.0); // Should be high variance
}

#[test]
fn test_quadtree_depth_limit() {
    let mut image = RgbaImage::new(8, 8);

    // Create high variance pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([((x * 32) % 256) as u8, ((y * 32) % 256) as u8, 0, 255]);
    }

    let tree = QuadTree::build(&image, 2, 1.0); // Very low threshold, limited depth

    // Should respect max depth
    let max_depth = tree.get_max_depth();
    assert!(max_depth <= 2);
}

#[test]
fn test_quadtree_quantization() {
    let mut image = RgbaImage::new(4, 4);

    // Create gradient
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([((x * 64) % 256) as u8, ((y * 64) % 256) as u8, 128, 255]);
    }

    let mut tree = QuadTree::build(&image, 3, 20.0);
    tree.quantize_with_palette(4);

    // Should assign palette indices
    let has_palette_indices = tree.has_palette_assignments();
    assert!(has_palette_indices);
}

#[test]
fn test_quadtree_to_grid_cells() {
    let mut image = RgbaImage::new(2, 2);

    // Simple 2x2 image
    image.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    image.put_pixel(1, 0, Rgba([0, 255, 0, 255]));
    image.put_pixel(0, 1, Rgba([0, 0, 255, 255]));
    image.put_pixel(1, 1, Rgba([255, 255, 0, 255]));

    let tree = QuadTree::build(&image, 3, 10.0);
    let cells = tree.to_grid_cells();

    // Should return grid cells with positions and colors
    assert!(!cells.is_empty());
    for (x, y, w, h, _color) in &cells {
        assert!(*x < 2);
        assert!(*y < 2);
        assert!(*w > 0);
        assert!(*h > 0);
    }
}

#[test]
fn test_quad_node_creation() {
    let node = QuadNode::new(0, 0, 4, Rgba([128, 128, 128, 255]), 25.0);

    assert_eq!(node.x, 0);
    assert_eq!(node.y, 0);
    assert_eq!(node.size, 4);
    assert_eq!(node.mean_color, Rgba([128, 128, 128, 255]));
    assert_eq!(node.variance, 25.0);
    assert!(node.palette_idx.is_none());
    assert!(node.children.is_none());
}

#[test]
fn test_quad_node_splitting() {
    let mut node = QuadNode::new(0, 0, 4, Rgba([128, 128, 128, 255]), 25.0);

    // Create child nodes
    let child1 = QuadNode::new(0, 0, 2, Rgba([255, 0, 0, 255]), 10.0);
    let child2 = QuadNode::new(2, 0, 2, Rgba([0, 255, 0, 255]), 10.0);
    let child3 = QuadNode::new(0, 2, 2, Rgba([0, 0, 255, 255]), 10.0);
    let child4 = QuadNode::new(2, 2, 2, Rgba([255, 255, 0, 255]), 10.0);

    node.children = Some(Box::new([child1, child2, child3, child4]));

    assert!(node.children.is_some());
    assert_eq!(node.children.as_ref().unwrap().len(), 4);
}

#[test]
fn test_image_region_creation() {
    let mut image = RgbaImage::new(4, 4);

    // Fill with test pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([x as u8 * 64, y as u8 * 64, 128, 255]);
    }

    let region = ImageRegion::new(&image, 0, 0, 4);

    assert_eq!(region.x, 0);
    assert_eq!(region.y, 0);
    assert_eq!(region.size, 4);
    assert!(!region.pixels.is_empty());
}

#[test]
fn test_variance_calculation() {
    let pixels = vec![
        Rgba([100, 100, 100, 255]),
        Rgba([200, 200, 200, 255]),
        Rgba([150, 150, 150, 255]),
    ];

    let (mean, variance) = calculate_region_variance(&pixels);

    // Should calculate correct mean and variance
    assert_eq!(mean, Rgba([150, 150, 150, 255]));
    assert!(variance > 0.0);
}

#[test]
fn test_should_split_decision() {
    let high_variance_node = QuadNode::new(0, 0, 4, Rgba([128, 128, 128, 255]), 100.0);
    let low_variance_node = QuadNode::new(0, 0, 4, Rgba([128, 128, 128, 255]), 5.0);

    assert!(should_split_node(&high_variance_node, 2, 50.0));
    assert!(!should_split_node(&low_variance_node, 2, 50.0));

    // Should not split if at max depth
    assert!(!should_split_node(&high_variance_node, 0, 50.0));
}

#[test]
fn test_quadtree_node_counting() {
    let mut image = RgbaImage::new(4, 4);

    // Create a pattern that will cause splitting
    for (x, _y, pixel) in image.enumerate_pixels_mut() {
        *pixel = if x < 2 {
            Rgba([255, 0, 0, 255])
        } else {
            Rgba([0, 255, 0, 255])
        };
    }

    let tree = QuadTree::build(&image, 3, 10.0);
    let node_count = tree.node_count();

    // Should have at least the root node
    assert!(node_count >= 1);

    // Node count should be reasonable for a 4x4 image with splitting
    assert!(node_count <= 85); // Maximum possible nodes for 3 levels: 1 + 4 + 16 + 64
}

#[test]
fn test_adaptive_progress_calculation() {
    let mut image = RgbaImage::new(8, 8);

    // Create varying detail levels in different quadrants
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if x < 4 && y < 4 {
            // High detail area
            *pixel = Rgba([((x * y * 64) % 256) as u8, ((x + y) * 32) as u8, 0, 255]);
        } else {
            // Low detail area
            *pixel = Rgba([128, 128, 128, 255]);
        }
    }

    let tree = QuadTree::build(&image, 4, 20.0);
    let node_count = tree.node_count();

    // Progress calculation should be based on actual node count
    assert!(node_count > 1); // Should have split due to high variance in one quadrant

    // Test that we can calculate progress percentages
    for processed in 0..=node_count {
        let progress_percent = (processed * 100) / node_count;
        assert!(progress_percent <= 100);
    }
}
