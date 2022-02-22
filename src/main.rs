use std::fs;
use std::process;
use std::time::Instant;

use ansi_term::Colour::Red;
use clap::Parser;
use distance::*;
use image::{DynamicImage, ImageBuffer, ImageError, io::Reader};
use lazy_static::lazy_static;
use rustdct::DctPlanner;

use log;

mod core;
mod logger;

// use num_bigint::BigUint;

const TEMP_FILE_DIR_PATH: &'static str = "./temp/";
const TEMP_FILE_EXT: &'static str = ".bmp";

const SCALED_SIDE_LENGTH: u32 = 256;
const SCALED_SIDE_LENGTH2: u32 = 32;

const TEMP_FILE_NAME_LANDSCAPE: &'static str = "landscape";
const TEMP_FILE_NAME_RESCALED: &'static str = "rescale";
const TEMP_FILE_NAME_RESCALED2: &'static str = "rescale2";
const TEMP_FILE_NAME_GREYSCALE: &'static str = "greyscale";
const TEMP_FILE_NAME_NORMALISED: &'static str = "normalised";
const TEMP_FILE_NAME_BINARY: &'static str = "binary";
const TEMP_FILE_NAME_DCT: &'static str = "dct";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long, default_value = "")]
	file_name: String,

	#[clap(long, default_value = "")]
	file_name2: String,

	#[clap(short, long)]
	verbose: bool,

	#[clap(short, long)]
	save_images: bool,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

fn main() {
	let start = Instant::now();

	if ARGS.file_name == "" && ARGS.file_name2 == "" {
		logger::print_error("Please specify \"file_name\"!");
		return;
	}
	if ARGS.file_name2 != "" {
		let hashes1 = core::hash(&ARGS.file_name, &true).clone();
		let hashes2 = core::hash(&ARGS.file_name2, &false).clone();
		logger::print_debug("Calculating Levenshtein distance.");
		let similarity = core::similarity(&hashes1, &hashes2);

		if ARGS.verbose {
			logger::print_debug(&format!(
				"Hash 1 (0deg): {}",
				core::to_hex(&hashes1[0])
			));
			logger::print_debug(&format!(
				"Hash 2 (0deg): {}",
				core::to_hex(&hashes2[0])
			));
			logger::print_debug(&format!(
				"Levenshtein distance between hashes: {}",
				similarity
			));
			logger::print_debug(&format!("Similarity score: {}", similarity));
		} else {
			logger::print_info(&format!("{}", similarity));
		}
	} else {
		let hashes1 = core::hash(&ARGS.file_name, &false).clone();

		if ARGS.verbose {
			logger::print_debug(&format!("Hash 1: {}", core::to_hex(&hashes1[0])));
		} else {
			logger::print_info(&core::to_hex(&hashes1[0]));
		}
	}
	let duration = start.elapsed();

	logger::print_debug(&format!("Finished job in: {:?}", duration));
}
