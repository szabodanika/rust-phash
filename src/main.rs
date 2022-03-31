use std::time::Instant;

use clap::Parser;
use lazy_static::lazy_static;

mod hashing;
mod logger;
mod tests;

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
	env_logger::init();

	let start = Instant::now();

	if ARGS.file_name == "" && ARGS.file_name2 == "" {
		logger::log_error("Please specify \"file_name\"!");
		return;
	}
	if ARGS.file_name2 != "" {
		let hashes1 = hashing::hash(&ARGS.file_name, &true).clone();
		let hashes2 = hashing::hash(&ARGS.file_name2, &false).clone();
		logger::log_debug("Calculating Levenshtein distance.");
		let similarity = hashing::similarity(&hashes1, &hashes2);

		if ARGS.verbose {
			logger::log_debug(&format!("Hash 1 (0deg): {}", hashing::to_hex(&hashes1[0])));
			logger::log_debug(&format!("Hash 2 (0deg): {}", hashing::to_hex(&hashes2[0])));
			logger::log_debug(&format!(
				"Levenshtein distance between hashes: {}",
				similarity
			));
			logger::log_debug(&format!("Similarity score: {}", similarity));
		} else {
			logger::log_info(&format!("{}", similarity));
		}
		println!("{}", similarity);
	} else {
		let hashes1 = hashing::hash(&ARGS.file_name, &false).clone();

		if ARGS.verbose {
			logger::log_debug(&format!("Hash 1: {}", hashing::to_hex(&hashes1[0])));
		} else {
			logger::log_info(&hashing::to_hex(&hashes1[0]));
		}
		println!("{}", &hashing::to_hex(&hashes1[0]));
	}
	let duration = start.elapsed();

	logger::log_debug(&format!("Finished job in: {:?}", duration));
}
