use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct GridVisualizer {
    #[allow(dead_code)]
    multi_progress: Arc<MultiProgress>,
    main_bar: ProgressBar,
    grid_bars: Vec<ProgressBar>,
    rows: u32,
    cols: u32,
    completed_count: Arc<Mutex<u32>>,
    start_time: Instant,
}

impl GridVisualizer {
    pub fn new(rows: u32, cols: u32) -> Self {
        let multi_progress = Arc::new(MultiProgress::new());
        let main_bar = multi_progress.add(ProgressBar::new((rows * cols) as u64));
        main_bar.set_style(create_main_progress_style());
        main_bar.set_message("Processing grid cells");

        // Enable steady tick for smoother updates
        main_bar.enable_steady_tick(std::time::Duration::from_millis(100));

        let mut grid_bars = Vec::new();
        for row in 0..rows {
            let bar = multi_progress.add(ProgressBar::new(cols as u64));
            bar.set_style(create_grid_progress_style());
            bar.set_prefix(format!("{row}"));
            grid_bars.push(bar);
        }

        // MultiProgress will handle rendering automatically

        Self {
            multi_progress,
            main_bar,
            grid_bars,
            rows,
            cols,
            completed_count: Arc::new(Mutex::new(0)),
            start_time: Instant::now(),
        }
    }

    pub fn update_cell(&self, row: u32, col: u32) {
        if row >= self.rows || col >= self.cols {
            panic!(
                "Invalid cell coordinates: ({}, {}), grid size: {}x{}",
                row, col, self.rows, self.cols
            );
        }

        // Update the specific row progress bar
        if let Some(bar) = self.grid_bars.get(row as usize) {
            bar.inc(1);
        }

        // Update main progress bar
        self.main_bar.inc(1);

        // Update completed count
        if let Ok(mut count) = self.completed_count.lock() {
            *count += 1;
        }
    }

    pub fn update_cell_with_message(&self, row: u32, col: u32, message: &str) {
        self.update_cell(row, col);
        self.main_bar.set_message(message.to_string());
    }

    pub fn finish(&self) {
        for bar in &self.grid_bars {
            bar.finish();
        }
        self.main_bar
            .finish_with_message("Grid processing completed");
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn cols(&self) -> u32 {
        self.cols
    }

    pub fn total_cells(&self) -> u32 {
        self.rows * self.cols
    }

    pub fn completed_cells(&self) -> u32 {
        match self.completed_count.lock() {
            Ok(count) => *count,
            Err(_) => 0,
        }
    }

    pub fn estimated_time_remaining(&self) -> Option<std::time::Duration> {
        let completed = self.completed_cells();
        if completed == 0 {
            return None;
        }

        let elapsed = self.start_time.elapsed();
        let total_cells = self.total_cells();
        let remaining_cells = total_cells - completed;

        if remaining_cells == 0 {
            return Some(std::time::Duration::from_secs(0));
        }

        let avg_time_per_cell = elapsed.as_secs_f64() / completed as f64;
        let estimated_remaining_secs = avg_time_per_cell * remaining_cells as f64;

        Some(std::time::Duration::from_secs_f64(estimated_remaining_secs))
    }

    pub fn display_grid_progress(&self) -> String {
        let mut grid = String::new();
        let completed = self.completed_cells();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell_index = row * self.cols + col;
                let cell_done = cell_index < completed;
                grid.push_str(if cell_done { "█" } else { "░" });
            }
            grid.push('\n');
        }
        grid
    }

    pub fn is_cell_complete(&self, row: u32, col: u32) -> bool {
        let completed = self.completed_cells();
        let cell_index = row * self.cols + col;
        cell_index < completed
    }
}

pub fn create_main_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}",
    )
    .unwrap_or_else(|_| ProgressStyle::default_bar())
}

pub fn create_grid_progress_style() -> ProgressStyle {
    ProgressStyle::with_template("Row {prefix:>3} [{bar:25.green/red}] {percent:>3}%")
        .unwrap_or_else(|_| ProgressStyle::default_bar())
}
