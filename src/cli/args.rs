use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pixel-art-rust")]
#[command(about = "Convert images to pixel art", long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Number of horizontal divisions")]
    pub width: u32,

    #[arg(long, help = "Number of vertical divisions")]
    pub height: u32,

    #[arg(short, long, help = "Input image path")]
    pub input: PathBuf,

    #[arg(short, long, help = "Output image path")]
    pub output: PathBuf,

    #[arg(
        short,
        long,
        default_value = "average",
        help = "Color extraction algorithm"
    )]
    pub algorithm: ColorAlgorithm,

    #[arg(short, long, help = "Number of colors for quantization")]
    pub colors: Option<u32>,

    #[arg(long, help = "Use adaptive quadtree instead of uniform grid")]
    pub adaptive: bool,

    #[arg(
        long,
        default_value = "10",
        help = "Max depth for quadtree (when --adaptive)"
    )]
    pub max_depth: u32,

    #[arg(
        long,
        default_value = "50.0",
        help = "Variance threshold for quadtree splitting"
    )]
    pub variance_threshold: f64,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum ColorAlgorithm {
    Average,
    #[value(name = "median-cut")]
    MedianCut,
    #[value(name = "kmeans")]
    KMeans,
}

impl Args {
    pub fn validate(&self) -> Result<()> {
        // Validate dimensions
        if self.width == 0 {
            return Err(anyhow::anyhow!("Width must be greater than 0"));
        }
        if self.height == 0 {
            return Err(anyhow::anyhow!("Height must be greater than 0"));
        }

        // Validate file paths
        if self.input.as_os_str().is_empty() {
            return Err(anyhow::anyhow!("Input file path cannot be empty"));
        }
        if self.output.as_os_str().is_empty() {
            return Err(anyhow::anyhow!("Output file path cannot be empty"));
        }

        // Validate colors parameter
        if let Some(colors) = self.colors {
            if colors == 0 {
                return Err(anyhow::anyhow!("Number of colors must be greater than 0"));
            }
            if colors > 256 {
                return Err(anyhow::anyhow!("Number of colors must be 256 or less"));
            }
        }

        // Validate quadtree parameters when adaptive is enabled
        if self.adaptive {
            if self.max_depth == 0 {
                return Err(anyhow::anyhow!(
                    "Max depth must be greater than 0 when using adaptive quadtree"
                ));
            }
            if self.max_depth > 20 {
                return Err(anyhow::anyhow!(
                    "Max depth must be 20 or less to avoid excessive memory usage"
                ));
            }
            if self.variance_threshold < 0.0 {
                return Err(anyhow::anyhow!("Variance threshold must be non-negative"));
            }
            if self.variance_threshold > 255.0 {
                return Err(anyhow::anyhow!("Variance threshold must be 255 or less"));
            }
        }

        Ok(())
    }
}
