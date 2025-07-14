use clap::Parser;
use pixel_art_rust::cli::args::*;
use std::path::PathBuf;

#[test]
fn test_args_parsing_valid_input() {
    let args = vec![
        "pixel-art-rust",
        "-w",
        "32",
        "--height",
        "24",
        "-i",
        "input.jpg",
        "-o",
        "output.png",
    ];

    let parsed = Args::try_parse_from(args);

    assert!(parsed.is_ok());
    let args = parsed.unwrap();

    assert_eq!(args.width, 32);
    assert_eq!(args.height, 24);
    assert_eq!(args.input, PathBuf::from("input.jpg"));
    assert_eq!(args.output, PathBuf::from("output.png"));
    assert_eq!(args.algorithm, ColorAlgorithm::Average);
    assert_eq!(args.colors, None);
    assert!(!args.adaptive);
    assert_eq!(args.max_depth, 10);
    assert!((args.variance_threshold - 50.0).abs() < 0.01);
}

#[test]
fn test_args_parsing_with_all_options() {
    let args = vec![
        "pixel-art-rust",
        "-w",
        "64",
        "--height",
        "48",
        "-i",
        "test.png",
        "-o",
        "result.jpg",
        "-a",
        "kmeans",
        "-c",
        "16",
        "--adaptive",
        "--max-depth",
        "8",
        "--variance-threshold",
        "30.0",
    ];

    let parsed = Args::try_parse_from(args);

    assert!(parsed.is_ok());
    let args = parsed.unwrap();

    assert_eq!(args.width, 64);
    assert_eq!(args.height, 48);
    assert_eq!(args.input, PathBuf::from("test.png"));
    assert_eq!(args.output, PathBuf::from("result.jpg"));
    assert_eq!(args.algorithm, ColorAlgorithm::KMeans);
    assert_eq!(args.colors, Some(16));
    assert!(args.adaptive);
    assert_eq!(args.max_depth, 8);
    assert!((args.variance_threshold - 30.0).abs() < 0.01);
}

#[test]
fn test_args_validation_invalid_dimensions() {
    let args = Args {
        width: 0,
        height: 32,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::Average,
        colors: None,
        adaptive: false,
        max_depth: 10,
        variance_threshold: 50.0,
    };

    let result = args.validate();
    assert!(result.is_err());

    let args = Args {
        width: 32,
        height: 0,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::Average,
        colors: None,
        adaptive: false,
        max_depth: 10,
        variance_threshold: 50.0,
    };

    let result = args.validate();
    assert!(result.is_err());
}

#[test]
fn test_args_validation_invalid_colors() {
    let args = Args {
        width: 32,
        height: 32,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::KMeans,
        colors: Some(0),
        adaptive: false,
        max_depth: 10,
        variance_threshold: 50.0,
    };

    let result = args.validate();
    assert!(result.is_err());

    let args = Args {
        width: 32,
        height: 32,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::KMeans,
        colors: Some(257),
        adaptive: false,
        max_depth: 10,
        variance_threshold: 50.0,
    };

    let result = args.validate();
    assert!(result.is_err());
}

#[test]
fn test_args_file_path_validation() {
    let args = Args {
        width: 32,
        height: 32,
        input: PathBuf::from(""),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::Average,
        colors: None,
        adaptive: false,
        max_depth: 10,
        variance_threshold: 50.0,
    };

    let result = args.validate();
    assert!(result.is_err());
}

#[test]
fn test_args_help_message_format() {
    let args = vec!["pixel-art-rust", "--help"];

    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // The error should contain help information
    let error = result.unwrap_err();
    let help_message = error.to_string();

    assert!(help_message.contains("pixel-art-rust"));
    assert!(help_message.contains("Convert images to pixel art"));
    assert!(help_message.contains("--width"));
    assert!(help_message.contains("--height"));
    assert!(help_message.contains("--input"));
    assert!(help_message.contains("--output"));
}

#[test]
fn test_color_algorithm_variants() {
    // Test all color algorithm variants
    let algorithms = vec![
        ("average", ColorAlgorithm::Average),
        ("median-cut", ColorAlgorithm::MedianCut),
        ("kmeans", ColorAlgorithm::KMeans),
    ];

    for (algo_str, expected_algo) in algorithms {
        let args = vec![
            "pixel-art-rust",
            "-w",
            "32",
            "--height",
            "32",
            "-i",
            "test.jpg",
            "-o",
            "out.png",
            "-a",
            algo_str,
        ];

        let parsed = Args::try_parse_from(args);
        assert!(parsed.is_ok());

        let args = parsed.unwrap();
        assert_eq!(args.algorithm, expected_algo);
    }
}

#[test]
fn test_adaptive_quadtree_options() {
    let args = vec![
        "pixel-art-rust",
        "-w",
        "32",
        "--height",
        "32",
        "-i",
        "test.jpg",
        "-o",
        "out.png",
        "--adaptive",
        "--max-depth",
        "5",
        "--variance-threshold",
        "25.5",
    ];

    let parsed = Args::try_parse_from(args);
    assert!(parsed.is_ok());

    let args = parsed.unwrap();
    assert!(args.adaptive);
    assert_eq!(args.max_depth, 5);
    assert!((args.variance_threshold - 25.5).abs() < 0.01);
}

#[test]
fn test_missing_required_args() {
    let test_cases = vec![
        // Missing width
        vec![
            "pixel-art-rust",
            "--height",
            "32",
            "-i",
            "test.jpg",
            "-o",
            "out.png",
        ],
        // Missing height
        vec![
            "pixel-art-rust",
            "-w",
            "32",
            "-i",
            "test.jpg",
            "-o",
            "out.png",
        ],
        // Missing input
        vec!["pixel-art-rust", "-w", "32", "-h", "32", "-o", "out.png"],
        // Missing output
        vec!["pixel-art-rust", "-w", "32", "-h", "32", "-i", "test.jpg"],
    ];

    for args in test_cases {
        let result = Args::try_parse_from(args);
        assert!(result.is_err());
    }
}

#[test]
fn test_validation_valid_args() {
    let args = Args {
        width: 32,
        height: 32,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::Average,
        colors: None,
        adaptive: false,
        max_depth: 10,
        variance_threshold: 50.0,
    };

    let result = args.validate();
    assert!(result.is_ok());
}

#[test]
fn test_validation_max_depth_constraints() {
    let args = Args {
        width: 32,
        height: 32,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::Average,
        colors: None,
        adaptive: true,
        max_depth: 0,
        variance_threshold: 50.0,
    };

    let result = args.validate();
    assert!(result.is_err());

    let args = Args {
        width: 32,
        height: 32,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::Average,
        colors: None,
        adaptive: true,
        max_depth: 21,
        variance_threshold: 50.0,
    };

    let result = args.validate();
    assert!(result.is_err());
}

#[test]
fn test_validation_variance_threshold_constraints() {
    let args = Args {
        width: 32,
        height: 32,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::Average,
        colors: None,
        adaptive: true,
        max_depth: 10,
        variance_threshold: -1.0,
    };

    let result = args.validate();
    assert!(result.is_err());

    let args = Args {
        width: 32,
        height: 32,
        input: PathBuf::from("test.jpg"),
        output: PathBuf::from("out.png"),
        algorithm: ColorAlgorithm::Average,
        colors: None,
        adaptive: true,
        max_depth: 10,
        variance_threshold: 1000.0,
    };

    let result = args.validate();
    assert!(result.is_err());
}
