#[cfg(test)]
mod tests {
	use std::fmt::format;
	use std::time::Instant;

	use crate::hashing;

	#[test]
	fn negative_test() {
		// for calculating average
		let mut sim_sum = 0f32;

		// test image folders (and image name prefixes)
		let image_names = ["airplane", "car", "cat",
			"dog", "flower", "fruit", "motorbike", "person"];

		// total number of comparisons will be (n * n)/2 * image_names.length
		let first_n_images = 4;
		let mut comparisons = 0;

		// for each image type
		for image_name in image_names {
			println!("Testing image set: {}", image_name);
			// for the first 100 images
			for i in 0..first_n_images {
				// take hash of one image
				let hashes1 = hashing::hash(
					&format!("./input/natural_images/{}/{}_{}.jpg",
									 image_name, image_name, &format!("{:0width$}", i, width = 4)),
					&true).clone();
				// compare it to other 99
				for l in i..first_n_images {
					if i == l {
						continue;
					} else {
						let hashes2 = hashing::hash(
							&format!("./input/natural_images/{}/{}_{}.jpg",
											 image_name, image_name, &format!("{:0width$}", l, width = 4)),
							&false).clone();

						let similarity = hashing::similarity(&hashes1, &hashes2);

						sim_sum += similarity;
						comparisons += 1;

						// check that similarity is below 0.75
						if (similarity > 0.75) {
							println!("!!! Sim < 0.75 = {} for: {} and {} ",
											 similarity,
											 &format!("./input/natural_images/{}/{}_{}.jpg",
																image_name, image_name, &format!("{:0width$}", i, width = 4)),
											 &format!("./input/natural_images/{}/{}_{}.jpg",
																image_name, image_name, &format!("{:0width$}", l, width = 4)));
						}
						// assert!(similarity < 0.75);
					}
				}
			}
		}

		let avg_sim = sim_sum / comparisons as f32;

		println!("Average similarity score ({} comparisons): {}",
						 comparisons,
						 avg_sim);

		assert!(avg_sim < 0.50);
	}

	#[test]
	fn positive_test() {
		// for calculating average
		let mut sim_sum = 0f32;

		// test image folders (and image name prefixes)
		let image_sets = [
			["1", "1-addition", "1-compression"],
			["3", "3-addition", "3-scaling"],
			["6", "6-90degrees", "6-crop"],
			["7", "7-crop-rotate", "7-hueshift"],
		];

		// total number of comparisons will be (n * n)/2 * image_names.length
		let mut comparisons = 0;

		// for each image set
		for image_set in image_sets {
			// take hash of first (original) image
			let hashes1 = hashing::hash(
				&format!("./input/building/building-img{}.jpeg",
								 image_set[0]), &true).clone();

			// compare it to others
			for l in 1..image_set.len() {
				let hashes2 = hashing::hash(
					&format!("./input/building/building-img{}.jpeg",
									 image_set[l]), &false).clone();

				let similarity = hashing::similarity(&hashes1, &hashes2);

				sim_sum += similarity;
				comparisons += 1;

				// check that similarity is above 90
				if (similarity < 0.90) {
					println!("!!! Sim < 0.90 = {} for: {} and {} ",
									 similarity,
									 &format!("./input/building/building-img{}.jpeg", image_set[0]),
									 &format!("./input/building/building-img{}.jpeg", image_set[l]));

					// assert!(similarity < 0.75);
				}
			}
		}

		let avg_sim = sim_sum / comparisons as f32;

		println!("Average similarity score ({} comparisons): {}",
						 comparisons,
						 avg_sim);

		assert!(avg_sim > 0.95);
	}

	#[test]
	fn time_test() {
		let start = Instant::now();

		// test image folders (and image name prefixes)
		let image_names = ["airplane", "car", "cat",
			"dog", "flower", "fruit", "motorbike", "person"];

		// total number of comparisons will be (n * n)/2 * image_names.length
		let first_n_images = 10;

		for image_name in image_names {
			for i in 0..first_n_images {
				hashing::hash(
					&format!("./input/natural_images/{}/{}_{}.jpg",
									 "airplane", "airplane", &format!("{:0width$}", i, width = 4)),
					&true);
			}
		}

		let duration = start.elapsed();

		let ms_per_hash = duration.as_millis() as usize / (image_names.len() * first_n_images);

		println!("Finished {} hash computations in {:?} : avg. {}ms/hash",
						 image_names.len() * first_n_images,
						 duration,
						 ms_per_hash);

		assert!(ms_per_hash < 100);
	}
}
