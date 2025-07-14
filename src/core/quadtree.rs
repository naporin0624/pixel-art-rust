use crate::core::color::calculate_color_variance;
use image::{Rgba, RgbaImage};

#[derive(Debug)]
pub struct QuadNode {
    pub x: u32,
    pub y: u32,
    pub size: u32,
    pub mean_color: Rgba<u8>,
    pub variance: f64,
    pub palette_idx: Option<u8>,
    pub children: Option<Box<[QuadNode; 4]>>,
}

impl QuadNode {
    pub fn new(x: u32, y: u32, size: u32, mean_color: Rgba<u8>, variance: f64) -> Self {
        Self {
            x,
            y,
            size,
            mean_color,
            variance,
            palette_idx: None,
            children: None,
        }
    }
}

pub struct QuadTree {
    pub root: QuadNode,
    pub max_depth: u32,
    pub variance_threshold: f64,
}

impl QuadTree {
    pub fn build(image: &RgbaImage, max_depth: u32, variance_threshold: f64) -> Self {
        let width = image.width();
        let height = image.height();
        let size = width.max(height);

        let region = ImageRegion::new(image, 0, 0, size);
        let root = Self::build_recursive(image, region, max_depth, variance_threshold);

        Self {
            root,
            max_depth,
            variance_threshold,
        }
    }

    fn build_recursive(
        image: &RgbaImage,
        region: ImageRegion,
        max_depth: u32,
        variance_threshold: f64,
    ) -> QuadNode {
        let (mean_color, variance) = calculate_region_variance(&region.pixels);

        let mut node = QuadNode::new(region.x, region.y, region.size, mean_color, variance);

        if should_split_node(&node, max_depth, variance_threshold) && region.size > 1 {
            let half_size = region.size / 2;

            let child_regions = [
                ImageRegion::new(image, region.x, region.y, half_size),
                ImageRegion::new(image, region.x + half_size, region.y, half_size),
                ImageRegion::new(image, region.x, region.y + half_size, half_size),
                ImageRegion::new(image, region.x + half_size, region.y + half_size, half_size),
            ];

            let children = [
                Self::build_recursive(
                    image,
                    child_regions[0].clone(),
                    max_depth - 1,
                    variance_threshold,
                ),
                Self::build_recursive(
                    image,
                    child_regions[1].clone(),
                    max_depth - 1,
                    variance_threshold,
                ),
                Self::build_recursive(
                    image,
                    child_regions[2].clone(),
                    max_depth - 1,
                    variance_threshold,
                ),
                Self::build_recursive(
                    image,
                    child_regions[3].clone(),
                    max_depth - 1,
                    variance_threshold,
                ),
            ];

            node.children = Some(Box::new(children));
        }

        node
    }

    pub fn get_max_depth(&self) -> u32 {
        self.get_node_depth(&self.root)
    }

    fn get_node_depth(&self, node: &QuadNode) -> u32 {
        if let Some(children) = &node.children {
            1 + children
                .iter()
                .map(Self::get_node_depth_static)
                .max()
                .unwrap_or(0)
        } else {
            0
        }
    }

    fn get_node_depth_static(node: &QuadNode) -> u32 {
        if let Some(children) = &node.children {
            1 + children
                .iter()
                .map(Self::get_node_depth_static)
                .max()
                .unwrap_or(0)
        } else {
            0
        }
    }

    pub fn quantize_with_palette(&mut self, _palette_size: u32) {
        // Simple implementation: assign palette indices to leaf nodes
        let mut palette_index = 0;
        Self::assign_palette_indices(&mut self.root, &mut palette_index);
    }

    fn assign_palette_indices(node: &mut QuadNode, palette_index: &mut u8) {
        if let Some(children) = &mut node.children {
            for child in children.iter_mut() {
                Self::assign_palette_indices(child, palette_index);
            }
        } else {
            node.palette_idx = Some(*palette_index);
            *palette_index = palette_index.wrapping_add(1);
        }
    }

    pub fn has_palette_assignments(&self) -> bool {
        self.node_has_palette_assignments(&self.root)
    }

    fn node_has_palette_assignments(&self, node: &QuadNode) -> bool {
        if let Some(children) = &node.children {
            children
                .iter()
                .any(Self::node_has_palette_assignments_static)
        } else {
            node.palette_idx.is_some()
        }
    }

    fn node_has_palette_assignments_static(node: &QuadNode) -> bool {
        if let Some(children) = &node.children {
            children
                .iter()
                .any(Self::node_has_palette_assignments_static)
        } else {
            node.palette_idx.is_some()
        }
    }

    pub fn node_count(&self) -> u32 {
        Self::count_nodes_recursive(&self.root)
    }

    fn count_nodes_recursive(node: &QuadNode) -> u32 {
        let mut count = 1;
        if let Some(children) = &node.children {
            for child in children.iter() {
                count += Self::count_nodes_recursive(child);
            }
        }
        count
    }

    pub fn traverse_with_callback<F>(&self, mut callback: F)
    where
        F: FnMut(&QuadNode),
    {
        Self::traverse_node_with_callback(&self.root, &mut callback);
    }

    fn traverse_node_with_callback<F>(node: &QuadNode, callback: &mut F)
    where
        F: FnMut(&QuadNode),
    {
        callback(node);
        if let Some(children) = &node.children {
            for child in children.iter() {
                Self::traverse_node_with_callback(child, callback);
            }
        }
    }

    pub fn to_grid_cells(&self) -> Vec<(u32, u32, u32, u32, Rgba<u8>)> {
        let mut cells = Vec::new();
        self.collect_leaf_cells(&self.root, &mut cells);
        cells
    }

    fn collect_leaf_cells(&self, node: &QuadNode, cells: &mut Vec<(u32, u32, u32, u32, Rgba<u8>)>) {
        if let Some(children) = &node.children {
            for child in children.iter() {
                Self::collect_leaf_cells_static(child, cells);
            }
        } else {
            cells.push((node.x, node.y, node.size, node.size, node.mean_color));
        }
    }

    fn collect_leaf_cells_static(node: &QuadNode, cells: &mut Vec<(u32, u32, u32, u32, Rgba<u8>)>) {
        if let Some(children) = &node.children {
            for child in children.iter() {
                Self::collect_leaf_cells_static(child, cells);
            }
        } else {
            cells.push((node.x, node.y, node.size, node.size, node.mean_color));
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageRegion {
    pub x: u32,
    pub y: u32,
    pub size: u32,
    pub pixels: Vec<Rgba<u8>>,
}

impl ImageRegion {
    pub fn new(image: &RgbaImage, x: u32, y: u32, size: u32) -> Self {
        let mut pixels = Vec::new();
        let img_width = image.width();
        let img_height = image.height();

        for dy in 0..size {
            for dx in 0..size {
                let px = x + dx;
                let py = y + dy;

                if px < img_width && py < img_height {
                    pixels.push(*image.get_pixel(px, py));
                }
            }
        }

        Self { x, y, size, pixels }
    }
}

pub fn calculate_region_variance(pixels: &[Rgba<u8>]) -> (Rgba<u8>, f64) {
    calculate_color_variance(pixels)
}

pub fn should_split_node(node: &QuadNode, max_depth: u32, variance_threshold: f64) -> bool {
    max_depth > 0 && node.variance > variance_threshold
}
