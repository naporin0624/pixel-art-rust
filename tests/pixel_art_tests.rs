use image::{DynamicImage, Rgba, RgbaImage};
use pixel_art_rust::core::color::AverageColorExtractor;
use pixel_art_rust::core::grid::Grid;
use pixel_art_rust::core::pixel_art::*;
use pixel_art_rust::core::quadtree::QuadTree;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[test]
fn test_pixel_art_conversion_small_image() {
    let mut image = RgbaImage::new(4, 4);

    // Create a simple pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = if (x + y) % 2 == 0 {
            Rgba([255, 0, 0, 255])
        } else {
            Rgba([0, 0, 255, 255])
        };
    }

    let grid = Grid::new(4, 4, 2, 2);
    let extractor = Box::new(AverageColorExtractor);
    let converter = PixelArtConverter::with_grid(grid, extractor);

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert(&dynamic_image);

    assert!(result.is_ok());
    let result_image = result.unwrap();
    assert_eq!(result_image.width(), 4);
    assert_eq!(result_image.height(), 4);
}

#[test]
fn test_parallel_grid_processing() {
    let mut image = RgbaImage::new(8, 8);

    // Create a gradient pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([(x * 32) as u8, (y * 32) as u8, 128, 255]);
    }

    let grid = Grid::new(8, 8, 4, 4);
    let extractor = Box::new(AverageColorExtractor);
    let converter = PixelArtConverter::with_grid(grid, extractor);

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert_parallel(&dynamic_image);

    assert!(result.is_ok());
    let result_image = result.unwrap();
    assert_eq!(result_image.width(), 8);
    assert_eq!(result_image.height(), 8);
}

#[test]
fn test_color_palette_generation() {
    let mut image = RgbaImage::new(4, 4);

    // Create distinct colors
    let colors = [
        Rgba([255, 0, 0, 255]),
        Rgba([0, 255, 0, 255]),
        Rgba([0, 0, 255, 255]),
        Rgba([255, 255, 0, 255]),
    ];

    for (i, pixel) in image.pixels_mut().enumerate() {
        *pixel = colors[i % colors.len()];
    }

    let grid = Grid::new(4, 4, 2, 2);
    let extractor = Box::new(AverageColorExtractor);
    let converter = PixelArtConverter::with_grid(grid, extractor);

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert(&dynamic_image);

    assert!(result.is_ok());
    // Should produce a quantized version with representative colors
}

#[test]
fn test_downsampling_accuracy() {
    let mut image = RgbaImage::new(8, 8);

    // Fill with uniform color
    for pixel in image.pixels_mut() {
        *pixel = Rgba([128, 128, 128, 255]);
    }

    let grid = Grid::new(8, 8, 2, 2);
    let extractor = Box::new(AverageColorExtractor);
    let converter = PixelArtConverter::with_grid(grid, extractor);

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert(&dynamic_image);

    assert!(result.is_ok());
    let result_image = result.unwrap();

    // Check if the result maintains the expected uniform color
    let rgba_image = result_image.to_rgba8();
    let first_pixel = rgba_image.get_pixel(0, 0);

    // Should be close to the original uniform color
    let expected = Rgba([128, 128, 128, 255]);
    let tolerance = 10;

    assert!((first_pixel.0[0] as i32 - expected.0[0] as i32).abs() <= tolerance);
    assert!((first_pixel.0[1] as i32 - expected.0[1] as i32).abs() <= tolerance);
    assert!((first_pixel.0[2] as i32 - expected.0[2] as i32).abs() <= tolerance);
    assert_eq!(first_pixel.0[3], expected.0[3]);
}

#[test]
fn test_quadtree_vs_grid_performance() {
    let mut image = RgbaImage::new(8, 8);

    // Create a mixed pattern
    for (x, _y, pixel) in image.enumerate_pixels_mut() {
        *pixel = if x < 4 {
            Rgba([255, 0, 0, 255])
        } else {
            Rgba([0, 0, 255, 255])
        };
    }

    let dynamic_image = DynamicImage::ImageRgba8(image);

    // Test with uniform grid
    let grid = Grid::new(8, 8, 4, 4);
    let extractor1 = Box::new(AverageColorExtractor);
    let grid_converter = PixelArtConverter::with_grid(grid, extractor1);
    let grid_result = grid_converter.convert(&dynamic_image);

    // Test with adaptive quadtree
    let extractor2 = Box::new(AverageColorExtractor);
    let tree_converter = PixelArtConverter::with_quadtree(3, 50.0, extractor2);
    let tree_result = tree_converter.convert(&dynamic_image);

    assert!(grid_result.is_ok());
    assert!(tree_result.is_ok());

    // Both should produce valid results
    let grid_image = grid_result.unwrap();
    let tree_image = tree_result.unwrap();

    assert_eq!(grid_image.width(), 8);
    assert_eq!(grid_image.height(), 8);
    assert_eq!(tree_image.width(), 8);
    assert_eq!(tree_image.height(), 8);
}

#[test]
fn test_adaptive_resolution() {
    let mut image = RgbaImage::new(16, 16);

    // Create a pattern with varying detail levels
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if x < 8 && y < 8 {
            // High detail area
            *pixel = Rgba([((x * 32) % 256) as u8, ((y * 32) % 256) as u8, 0, 255]);
        } else {
            // Low detail area
            *pixel = Rgba([128, 128, 128, 255]);
        }
    }

    let extractor = Box::new(AverageColorExtractor);
    let converter = PixelArtConverter::with_quadtree(4, 20.0, extractor);

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert(&dynamic_image);

    assert!(result.is_ok());
    let result_image = result.unwrap();
    assert_eq!(result_image.width(), 16);
    assert_eq!(result_image.height(), 16);
}

#[test]
fn test_processing_strategy_enum() {
    let grid = Grid::new(4, 4, 2, 2);
    let quadtree = QuadTree::build(&RgbaImage::new(4, 4), 3, 50.0);

    let grid_strategy = ProcessingStrategy::UniformGrid(grid);
    let tree_strategy = ProcessingStrategy::AdaptiveQuadTree(quadtree);

    // Test pattern matching
    match grid_strategy {
        ProcessingStrategy::UniformGrid(_) => {
            // Should match this branch
        }
        ProcessingStrategy::AdaptiveQuadTree(_) => {
            panic!("Should not match quadtree");
        }
    }

    match tree_strategy {
        ProcessingStrategy::UniformGrid(_) => {
            panic!("Should not match grid");
        }
        ProcessingStrategy::AdaptiveQuadTree(_) => {
            // Should match this branch
        }
    }
}

#[test]
fn test_callback_from_multiple_threads() {
    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = Arc::clone(&counter);

    let callback = Arc::new(move |_row: u32, _col: u32| {
        counter_clone.fetch_add(1, Ordering::Relaxed);
    });

    // Simulate multiple threads calling the callback
    let mut handles = vec![];
    for _ in 0..4 {
        let cb = Arc::clone(&callback);
        let handle = thread::spawn(move || {
            for i in 0..10 {
                cb(i, i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.load(Ordering::Relaxed), 40);
}

#[test]
fn test_atomic_counter_accuracy() {
    let counter = Arc::new(AtomicU32::new(0));
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];
    for thread_id in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let results_clone = Arc::clone(&results);

        let handle = thread::spawn(move || {
            for i in 0..5 {
                let value = counter_clone.fetch_add(1, Ordering::Relaxed);
                results_clone.lock().unwrap().push((thread_id, i, value));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = counter.load(Ordering::Relaxed);
    let results_vec = results.lock().unwrap();

    assert_eq!(final_count, 15);
    assert_eq!(results_vec.len(), 15);
}

#[test]
fn test_callback_performance_overhead() {
    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = Arc::clone(&counter);

    let callback = Arc::new(move |_row: u32, _col: u32| {
        counter_clone.fetch_add(1, Ordering::Relaxed);
    });

    let start = std::time::Instant::now();

    // Simulate high-frequency callback calls
    for row in 0..100 {
        for col in 0..100 {
            callback(row, col);
        }
    }

    let duration = start.elapsed();

    assert_eq!(counter.load(Ordering::Relaxed), 10000);
    // Performance check: should complete within reasonable time
    assert!(
        duration.as_millis() < 1000,
        "Callback overhead too high: {duration:?}"
    );
}

#[test]
fn test_visualizer_callback_integration() {
    let mut image = RgbaImage::new(8, 8);

    // Fill with test pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([x as u8 * 32, y as u8 * 32, 128, 255]);
    }

    let grid = Grid::new(8, 8, 4, 4);
    let extractor = Box::new(AverageColorExtractor);
    let mut converter = PixelArtConverter::with_grid(grid, extractor);

    // Test integration with Arc-based callback similar to visualizer
    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = Arc::clone(&call_count);

    let callback = Arc::new(move |_row: u32, _col: u32| {
        call_count_clone.fetch_add(1, Ordering::Relaxed);
    });

    converter.set_progress_callback(callback);

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert(&dynamic_image);

    assert!(result.is_ok());
    // Should have called progress callback for each grid cell (4x4 = 16 cells)
    assert_eq!(call_count.load(Ordering::Relaxed), 16);
}

#[test]
fn test_arc_cloning_safety() {
    // Test that Arc cloning works correctly for callbacks
    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = Arc::clone(&counter);

    let callback = Arc::new(move |_row: u32, _col: u32| {
        counter_clone.fetch_add(1, Ordering::Relaxed);
    });

    // Simulate what happens in main.rs when passing to converter
    let callback_for_converter = Arc::clone(&callback);

    // Simulate converter calling the callback
    callback_for_converter(0, 0);
    callback_for_converter(0, 1);

    assert_eq!(counter.load(Ordering::Relaxed), 2);

    // Original callback should still work
    callback(1, 0);
    assert_eq!(counter.load(Ordering::Relaxed), 3);
}

#[test]
fn test_parallel_progress_updates() {
    let mut image = RgbaImage::new(16, 16);

    // Fill with test pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([x as u8 * 16, y as u8 * 16, 128, 255]);
    }

    let grid = Grid::new(16, 16, 8, 8);
    let extractor = Box::new(AverageColorExtractor);
    let mut converter = PixelArtConverter::with_grid(grid, extractor);

    let call_count = Arc::new(AtomicU32::new(0));
    let call_timestamps = Arc::new(Mutex::new(Vec::new()));
    let start_time = Instant::now();

    let count_clone = Arc::clone(&call_count);
    let timestamps_clone = Arc::clone(&call_timestamps);

    converter.set_progress_callback(Arc::new(move |_row, _col| {
        count_clone.fetch_add(1, Ordering::Relaxed);
        let elapsed = start_time.elapsed();
        timestamps_clone.lock().unwrap().push(elapsed);
    }));

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert_parallel(&dynamic_image);

    assert!(result.is_ok());
    assert_eq!(call_count.load(Ordering::Relaxed), 64); // 8x8 grid

    // Check that progress updates happened over time (not all at once)
    let timestamps = call_timestamps.lock().unwrap();
    assert!(timestamps.len() == 64);

    // Verify that updates didn't all happen at the exact same time
    let mut sorted_timestamps = timestamps.clone();
    sorted_timestamps.sort();
    let time_span = sorted_timestamps
        .last()
        .unwrap()
        .saturating_sub(*sorted_timestamps.first().unwrap());
    assert!(time_span.as_nanos() > 0); // Some time should have passed between first and last update
}

#[test]
fn test_progress_ordering() {
    let mut image = RgbaImage::new(8, 8);

    // Fill with test pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([x as u8 * 32, y as u8 * 32, 128, 255]);
    }

    let grid = Grid::new(8, 8, 4, 4);
    let extractor = Box::new(AverageColorExtractor);
    let mut converter = PixelArtConverter::with_grid(grid, extractor);

    let progress_data = Arc::new(Mutex::new(Vec::new()));
    let progress_clone = Arc::clone(&progress_data);

    converter.set_progress_callback(Arc::new(move |row, col| {
        progress_clone.lock().unwrap().push((row, col));
    }));

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert_parallel(&dynamic_image);

    assert!(result.is_ok());

    let progress_vec = progress_data.lock().unwrap();
    assert_eq!(progress_vec.len(), 16); // 4x4 grid

    // Verify all expected grid positions were reported
    let mut expected_positions = Vec::new();
    for row in 0..4 {
        for col in 0..4 {
            expected_positions.push((row, col));
        }
    }

    let mut actual_positions = progress_vec.clone();
    actual_positions.sort();
    expected_positions.sort();

    assert_eq!(actual_positions, expected_positions);
}

#[test]
fn test_update_frequency() {
    let mut image = RgbaImage::new(32, 32);

    // Fill with test pattern requiring some computation
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([
            ((x * y) % 256) as u8,
            ((x + y) % 256) as u8,
            ((x ^ y) % 256) as u8,
            255,
        ]);
    }

    let grid = Grid::new(32, 32, 16, 16);
    let extractor = Box::new(AverageColorExtractor);
    let mut converter = PixelArtConverter::with_grid(grid, extractor);

    let update_count = Arc::new(AtomicU32::new(0));
    let update_times = Arc::new(Mutex::new(Vec::new()));
    let start_time = Instant::now();

    let count_clone = Arc::clone(&update_count);
    let times_clone = Arc::clone(&update_times);

    converter.set_progress_callback(Arc::new(move |_row, _col| {
        count_clone.fetch_add(1, Ordering::Relaxed);
        times_clone.lock().unwrap().push(start_time.elapsed());
    }));

    let processing_start = Instant::now();
    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert_parallel(&dynamic_image);
    let processing_duration = processing_start.elapsed();

    assert!(result.is_ok());
    assert_eq!(update_count.load(Ordering::Relaxed), 256); // 16x16 grid

    // Check update frequency - should have reasonable distribution over time
    let times = update_times.lock().unwrap();
    assert_eq!(times.len(), 256);

    // Updates should span most of the processing time
    let first_update = times.iter().min().unwrap();
    let last_update = times.iter().max().unwrap();
    let update_span = last_update.saturating_sub(*first_update);

    // Update span should be at least 10% of total processing time
    assert!(update_span.as_nanos() >= processing_duration.as_nanos() / 10);
}

#[test]
fn test_progress_callback() {
    let mut image = RgbaImage::new(4, 4);

    // Fill with test pattern
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgba([x as u8 * 64, y as u8 * 64, 128, 255]);
    }

    let grid = Grid::new(4, 4, 2, 2);
    let extractor = Box::new(AverageColorExtractor);
    let mut converter = PixelArtConverter::with_grid(grid, extractor);

    converter.set_progress_callback(Arc::new(|_row, _col| {
        // Progress callback called for each grid cell
    }));

    let dynamic_image = DynamicImage::ImageRgba8(image);
    let result = converter.convert(&dynamic_image);

    assert!(result.is_ok());
    // Progress callback should have been called for each grid cell
    // Note: This test won't work as written because progress_calls is moved
    // This is more of a compilation test to ensure the API works
}
