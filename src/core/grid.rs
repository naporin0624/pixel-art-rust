#[derive(Debug, Clone)]
pub struct Grid {
    width: u32,
    height: u32,
    cell_width: u32,
    cell_height: u32,
}

impl Grid {
    pub fn new(image_width: u32, image_height: u32, grid_width: u32, grid_height: u32) -> Self {
        let cell_width = if grid_width > 0 {
            image_width / grid_width
        } else {
            0
        };
        let cell_height = if grid_height > 0 {
            image_height / grid_height
        } else {
            0
        };

        Self {
            width: grid_width,
            height: grid_height,
            cell_width,
            cell_height,
        }
    }

    pub fn get_cell_bounds(&self, row: u32, col: u32) -> (u32, u32, u32, u32) {
        let x = col * self.cell_width;
        let y = row * self.cell_height;
        (x, y, self.cell_width, self.cell_height)
    }

    pub fn cell_count(&self) -> u32 {
        self.width * self.height
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (u32, u32)> {
        (0..self.height).flat_map(move |row| (0..self.width).map(move |col| (row, col)))
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cell_width(&self) -> u32 {
        self.cell_width
    }

    pub fn cell_height(&self) -> u32 {
        self.cell_height
    }
}
