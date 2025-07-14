use anyhow::{Context, Result};
use clap::Parser;
use pixel_art_rust::cli::args::{Args, ColorAlgorithm};
use pixel_art_rust::cli::visualizer::GridVisualizer;
use pixel_art_rust::core::color::{AverageColorExtractor, KMeansExtractor, MedianCutExtractor};
use pixel_art_rust::core::grid::Grid;
use pixel_art_rust::core::pixel_art::PixelArtConverter;
use std::sync::Arc;

fn main() -> Result<()> {
    let args = Args::parse();
    args.validate().context("Invalid arguments")?;

    println!("Loading image: {:?}", args.input);
    let image = image::open(&args.input)
        .with_context(|| format!("Failed to open image: {:?}", args.input))?;

    println!("Image loaded: {}x{}", image.width(), image.height());

    let visualizer = if args.adaptive {
        // For quadtree, we don't know the exact grid size, so use a reasonable default
        Arc::new(GridVisualizer::new(args.height, args.width))
    } else {
        Arc::new(GridVisualizer::new(args.height, args.width))
    };

    let mut converter = if args.adaptive {
        println!(
            "Using adaptive quadtree processing (depth: {}, threshold: {})",
            args.max_depth, args.variance_threshold
        );
        PixelArtConverter::with_quadtree(
            args.max_depth,
            args.variance_threshold,
            create_color_extractor(&args)?,
        )
    } else {
        println!(
            "Using uniform grid processing ({}x{})",
            args.width, args.height
        );
        let grid = Grid::new(image.width(), image.height(), args.width, args.height);
        PixelArtConverter::with_grid(grid, create_color_extractor(&args)?)
    };

    // Clone visualizer Arc for the callback
    let vis_callback = Arc::clone(&visualizer);
    converter.set_progress_callback(Arc::new(move |row, col| {
        vis_callback.update_cell(row, col);
    }));

    println!("Converting image to pixel art...");
    let pixel_art = converter
        .convert_parallel(&image)
        .context("Failed to convert image")?;

    println!("Saving result to: {:?}", args.output);
    pixel_art
        .save(&args.output)
        .with_context(|| format!("Failed to save image: {:?}", args.output))?;

    visualizer.finish();
    println!("Conversion completed successfully!");
    Ok(())
}

fn create_color_extractor(
    args: &Args,
) -> Result<Box<dyn pixel_art_rust::core::color::ColorExtractor>> {
    match args.algorithm {
        ColorAlgorithm::Average => Ok(Box::new(AverageColorExtractor)),
        ColorAlgorithm::MedianCut => {
            let max_colors = args.colors.unwrap_or(16);
            Ok(Box::new(MedianCutExtractor { max_colors }))
        }
        ColorAlgorithm::KMeans => {
            let k = args.colors.unwrap_or(16);
            Ok(Box::new(KMeansExtractor {
                k,
                max_iterations: 10,
            }))
        }
    }
}
