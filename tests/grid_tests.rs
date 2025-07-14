use pixel_art_rust::core::grid::Grid;

#[test]
fn test_grid_creation_with_valid_dimensions() {
    let grid = Grid::new(800, 600, 32, 24);
    assert_eq!(grid.width(), 32);
    assert_eq!(grid.height(), 24);
    assert_eq!(grid.cell_width(), 800 / 32);
    assert_eq!(grid.cell_height(), 600 / 24);
}

#[test]
fn test_grid_cell_bounds_calculation() {
    let grid = Grid::new(800, 600, 4, 3);

    // Test first cell (0, 0)
    let (x, y, w, h) = grid.get_cell_bounds(0, 0);
    assert_eq!(x, 0);
    assert_eq!(y, 0);
    assert_eq!(w, 200);
    assert_eq!(h, 200);

    // Test middle cell (1, 1)
    let (x, y, w, h) = grid.get_cell_bounds(1, 1);
    assert_eq!(x, 200);
    assert_eq!(y, 200);
    assert_eq!(w, 200);
    assert_eq!(h, 200);

    // Test last cell (2, 3)
    let (x, y, w, h) = grid.get_cell_bounds(2, 3);
    assert_eq!(x, 600);
    assert_eq!(y, 400);
    assert_eq!(w, 200);
    assert_eq!(h, 200);
}

#[test]
fn test_grid_cell_index_to_coordinates() {
    let grid = Grid::new(800, 600, 4, 3);

    let mut cells: Vec<(u32, u32)> = grid.iter_cells().collect();
    cells.sort();

    // Check that we get all expected coordinates
    let expected = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 0),
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 0),
        (2, 1),
        (2, 2),
        (2, 3),
    ];

    assert_eq!(cells, expected);
    assert_eq!(grid.cell_count(), 12);
}

#[test]
fn test_grid_boundaries_edge_cases() {
    // Test with image dimensions not evenly divisible by grid
    let grid = Grid::new(801, 601, 4, 3);

    let (x, y, w, h) = grid.get_cell_bounds(0, 0);
    assert_eq!(x, 0);
    assert_eq!(y, 0);
    assert_eq!(w, 801 / 4);
    assert_eq!(h, 601 / 3);

    // Test with 1x1 grid
    let grid = Grid::new(100, 100, 1, 1);
    let (x, y, w, h) = grid.get_cell_bounds(0, 0);
    assert_eq!(x, 0);
    assert_eq!(y, 0);
    assert_eq!(w, 100);
    assert_eq!(h, 100);
    assert_eq!(grid.cell_count(), 1);

    // Test with grid larger than image
    let grid = Grid::new(10, 10, 20, 20);
    let (x, y, w, h) = grid.get_cell_bounds(0, 0);
    assert_eq!(x, 0);
    assert_eq!(y, 0);
    assert_eq!(w, 0);
    assert_eq!(h, 0);
}
