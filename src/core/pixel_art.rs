use crate::core::color::ColorExtractor;
use crate::core::grid::Grid;
use crate::core::quadtree::QuadTree;
use anyhow::Result;
use image::{DynamicImage, Rgba, RgbaImage};
use rayon::prelude::*;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

thread_local! {
    static PIXEL_BUFFER: RefCell<Vec<Rgba<u8>>> = RefCell::new(Vec::with_capacity(65536));
}

#[allow(dead_code)]
struct PixelDataSoA {
    r: Vec<u8>,
    g: Vec<u8>,
    b: Vec<u8>,
    a: Vec<u8>,
}

#[allow(dead_code)]
impl PixelDataSoA {
    fn from_rgba_slice(pixels: &[Rgba<u8>]) -> Self {
        let len = pixels.len();
        let mut r = Vec::with_capacity(len);
        let mut g = Vec::with_capacity(len);
        let mut b = Vec::with_capacity(len);
        let mut a = Vec::with_capacity(len);

        for pixel in pixels {
            r.push(pixel.0[0]);
            g.push(pixel.0[1]);
            b.push(pixel.0[2]);
            a.push(pixel.0[3]);
        }

        Self { r, g, b, a }
    }

    fn len(&self) -> usize {
        self.r.len()
    }

    fn average_color(&self) -> Rgba<u8> {
        if self.len() == 0 {
            return Rgba([0, 0, 0, 255]);
        }

        let sum_r: u32 = self.r.iter().map(|&x| x as u32).sum();
        let sum_g: u32 = self.g.iter().map(|&x| x as u32).sum();
        let sum_b: u32 = self.b.iter().map(|&x| x as u32).sum();

        let count = self.len() as u32;
        Rgba([
            (sum_r / count) as u8,
            (sum_g / count) as u8,
            (sum_b / count) as u8,
            255,
        ])
    }
}

pub enum ProcessingStrategy {
    UniformGrid(Grid),
    AdaptiveQuadTree(QuadTree),
}

pub struct PixelArtConverter {
    strategy: ProcessingStrategy,
    color_extractor: Box<dyn ColorExtractor>,
    progress_callback: Option<Arc<dyn Fn(u32, u32) + Send + Sync>>,
}

impl PixelArtConverter {
    pub fn with_grid(grid: Grid, extractor: Box<dyn ColorExtractor>) -> Self {
        Self {
            strategy: ProcessingStrategy::UniformGrid(grid),
            color_extractor: extractor,
            progress_callback: None,
        }
    }

    pub fn with_quadtree(
        max_depth: u32,
        variance_threshold: f64,
        extractor: Box<dyn ColorExtractor>,
    ) -> Self {
        // Create a dummy image for now - in real implementation, this would be set during conversion
        let dummy_image = RgbaImage::new(1, 1);
        let quadtree = QuadTree::build(&dummy_image, max_depth, variance_threshold);

        Self {
            strategy: ProcessingStrategy::AdaptiveQuadTree(quadtree),
            color_extractor: extractor,
            progress_callback: None,
        }
    }

    pub fn set_progress_callback(&mut self, callback: Arc<dyn Fn(u32, u32) + Send + Sync>) {
        self.progress_callback = Some(callback);
    }

    pub fn convert(&self, image: &DynamicImage) -> Result<DynamicImage> {
        match &self.strategy {
            ProcessingStrategy::UniformGrid(grid) => self.process_with_grid(image, grid),
            ProcessingStrategy::AdaptiveQuadTree(_) => self.process_with_quadtree(image),
        }
    }

    pub fn convert_parallel(&self, image: &DynamicImage) -> Result<DynamicImage> {
        match &self.strategy {
            ProcessingStrategy::UniformGrid(grid) => self.process_with_grid_parallel(image, grid),
            ProcessingStrategy::AdaptiveQuadTree(_) => self.process_with_quadtree(image),
        }
    }

    fn process_with_grid(&self, image: &DynamicImage, grid: &Grid) -> Result<DynamicImage> {
        let width = image.width();
        let height = image.height();
        let mut result_data = vec![0u8; (width * height * 4) as usize];

        for (row, col) in grid.iter_cells() {
            let color = self.process_grid_cell(image, row, col, grid);
            let (x, y, w, h) = grid.get_cell_bounds(row, col);

            // Fill the cell with the extracted color using direct slice manipulation
            for dy in 0..h {
                let current_y = y + dy;
                if current_y >= height {
                    break;
                }

                let row_start = (current_y * width + x) as usize * 4;
                let max_dx = (width - x).min(w);

                for dx in 0..max_dx {
                    let idx = row_start + (dx as usize * 4);
                    result_data[idx] = color.0[0]; // R
                    result_data[idx + 1] = color.0[1]; // G
                    result_data[idx + 2] = color.0[2]; // B
                    result_data[idx + 3] = color.0[3]; // A
                }
            }

            if let Some(callback) = &self.progress_callback {
                callback(row, col);
            }
        }

        let result_image = RgbaImage::from_raw(width, height, result_data).unwrap();
        Ok(DynamicImage::ImageRgba8(result_image))
    }

    fn process_with_grid_parallel(
        &self,
        image: &DynamicImage,
        grid: &Grid,
    ) -> Result<DynamicImage> {
        let width = image.width();
        let height = image.height();
        let mut result_data = vec![0u8; (width * height * 4) as usize];

        let cells: Vec<(u32, u32)> = grid.iter_cells().collect();
        let progress_counter = Arc::new(AtomicU32::new(0));

        let processed_cells: Vec<(u32, u32, Rgba<u8>)> = cells
            .par_iter()
            .map(|(row, col)| {
                let color = self.process_grid_cell(image, *row, *col, grid);

                // Real-time progress update
                if let Some(callback) = &self.progress_callback {
                    callback(*row, *col);
                }

                let completed = progress_counter.fetch_add(1, Ordering::Relaxed) + 1;
                if completed % 10 == 0 {
                    // Throttle updates to avoid overwhelming the UI
                    std::thread::yield_now();
                }

                (*row, *col, color)
            })
            .collect();

        for (row, col, color) in processed_cells {
            let (x, y, w, h) = grid.get_cell_bounds(row, col);

            // Fill the cell with the extracted color using direct slice manipulation
            for dy in 0..h {
                let current_y = y + dy;
                if current_y >= height {
                    break;
                }

                let row_start = (current_y * width + x) as usize * 4;
                let max_dx = (width - x).min(w);

                for dx in 0..max_dx {
                    let idx = row_start + (dx as usize * 4);
                    result_data[idx] = color.0[0]; // R
                    result_data[idx + 1] = color.0[1]; // G
                    result_data[idx + 2] = color.0[2]; // B
                    result_data[idx + 3] = color.0[3]; // A
                }
            }
        }

        let result_image = RgbaImage::from_raw(width, height, result_data).unwrap();
        Ok(DynamicImage::ImageRgba8(result_image))
    }

    fn process_with_quadtree(&self, image: &DynamicImage) -> Result<DynamicImage> {
        let rgba_image = image.to_rgba8();
        let quadtree = QuadTree::build(&rgba_image, 4, 50.0);

        // Add progress callback support for quadtree processing
        if let Some(callback) = &self.progress_callback {
            let node_count = quadtree.node_count();
            let processed = Arc::new(AtomicU32::new(0));
            let callback_clone = Arc::clone(callback);
            let processed_clone = Arc::clone(&processed);

            quadtree.traverse_with_callback(|_node| {
                let current = processed_clone.fetch_add(1, Ordering::Relaxed);
                // Map node progress to grid coordinates for visualization
                let progress_percent = (current * 100) / node_count;
                let estimated_row = progress_percent / 10;
                let estimated_col = progress_percent % 10;
                callback_clone(estimated_row, estimated_col);
            });
        }

        let result_image = self.render_quadtree_to_image(&quadtree, image.width(), image.height());
        Ok(DynamicImage::ImageRgba8(result_image))
    }

    fn process_grid_cell(&self, image: &DynamicImage, row: u32, col: u32, grid: &Grid) -> Rgba<u8> {
        let bounds = grid.get_cell_bounds(row, col);
        let rgba_image = image.to_rgba8();

        PIXEL_BUFFER.with(|buffer| {
            let mut buffer = buffer.borrow_mut();
            self.extract_cell_pixels_fast(&rgba_image, bounds, &mut buffer);
            self.color_extractor.extract_color(&buffer)
        })
    }

    fn extract_cell_pixels_fast(
        &self,
        image: &RgbaImage,
        bounds: (u32, u32, u32, u32),
        buffer: &mut Vec<Rgba<u8>>,
    ) {
        let (x, y, w, h) = bounds;
        buffer.clear();
        buffer.reserve((w * h) as usize);

        // Use raw pointer access to avoid bounds checks
        let img_width = image.width();
        let img_height = image.height();
        let pixels = image.as_raw();

        for dy in 0..h {
            let current_y = y + dy;
            if current_y >= img_height {
                break;
            }

            let row_start = (current_y * img_width + x) as usize * 4;
            let max_dx = (img_width - x).min(w);

            for dx in 0..max_dx {
                let idx = row_start + (dx as usize * 4);
                unsafe {
                    let r = *pixels.get_unchecked(idx);
                    let g = *pixels.get_unchecked(idx + 1);
                    let b = *pixels.get_unchecked(idx + 2);
                    let a = *pixels.get_unchecked(idx + 3);
                    buffer.push(Rgba([r, g, b, a]));
                }
            }
        }
    }

    fn render_quadtree_to_image(&self, tree: &QuadTree, width: u32, height: u32) -> RgbaImage {
        let mut result_image = RgbaImage::new(width, height);
        let cells = tree.to_grid_cells();

        for (x, y, w, h, color) in cells {
            for dy in 0..h {
                for dx in 0..w {
                    if x + dx < width && y + dy < height {
                        result_image.put_pixel(x + dx, y + dy, color);
                    }
                }
            }
        }

        result_image
    }
}
