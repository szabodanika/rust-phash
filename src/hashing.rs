use crate::logger;
use distance::*;
use image::{io::Reader, DynamicImage, ImageBuffer, ImageError};
use rustdct::DctPlanner;
use std::fs;
use std::process;
// use num_bigint::BigUint;

const TEMP_FILE_DIR_PATH: &'static str = "./temp/";
const TEMP_FILE_EXT: &'static str = ".bmp";

const SCALED_SIDE_LENGTH: u32 = 64;
const SCALED_SIDE_LENGTH2: u32 = 8;

const TEMP_FILE_NAME_LANDSCAPE: &'static str = "landscape";
const TEMP_FILE_NAME_RESCALED: &'static str = "rescale";
const TEMP_FILE_NAME_RESCALED2: &'static str = "rescale2";
const TEMP_FILE_NAME_GREYSCALE: &'static str = "greyscale";
const TEMP_FILE_NAME_NORMALISED: &'static str = "normalised";
const TEMP_FILE_NAME_BINARY: &'static str = "binary";
const TEMP_FILE_NAME_DCT: &'static str = "dct";

pub fn similarity(hashes1: &Vec<String>, hashes2: &Vec<String>) -> f32 {
    1f32 - lowest_levenshtein_distance(hashes1, hashes2)
}

// hashes 1 contains 4 rotations, hashes 2 contains only 1
fn lowest_levenshtein_distance(hashes1: &Vec<String>, hashes2: &Vec<String>) -> f32 {
    let mut lowest: f32 = 1f32;
    for i in 0..hashes1.len() {
        let dist = hash_distance(&hashes1[i], &hashes2[0]);
        if dist < lowest {
            lowest = dist;
        }
    }
    lowest
}

pub fn hash_similarity(hash1: &str, hash2: &str) -> f32 {
    1f32 - hash_distance(hash1, hash2)
}

fn hash_distance(hash1: &str, hash2: &str) -> f32 {
    levenshtein(hash1, hash2) as f32 / hash1.len() as f32
}

pub fn hash(file_name: &str, rotations: &bool) -> Vec<String> {
    // STEP 1 - LOAD IMAGE
    logger::log_debug(&format!("Loading {}.", file_name));
    let load_result = load_from_disk(&file_name);

    let image = load_result.unwrap();
    save_to_disk(&image, TEMP_FILE_NAME_LANDSCAPE);

    // STEP 2 - DOWNSCALE IMAGE
    let rescaled = rescale_image(&image, SCALED_SIDE_LENGTH, SCALED_SIDE_LENGTH);
    save_to_disk(&rescaled, TEMP_FILE_NAME_RESCALED);

    // STEP 3 - GREYSCALE
    let greyscale = greyscale(&rescaled);
    save_to_disk(&greyscale, TEMP_FILE_NAME_GREYSCALE);

    // STEP 4 - NORMALISATION
    let normalised = normalise(&greyscale);
    save_to_disk(&normalised, TEMP_FILE_NAME_NORMALISED);

    // STEP 5 - APPLY DCT
    let dct = discrete_cosine_transform(&normalised, 8);
    save_to_disk(&dct, TEMP_FILE_NAME_DCT);

    // STEP 6 - DOWNSCALE AGAIN
    let rescaled2 = rescale_image(&dct, SCALED_SIDE_LENGTH2, SCALED_SIDE_LENGTH2);
    save_to_disk(&rescaled2, TEMP_FILE_NAME_RESCALED2);

    // OPTIONAL STEP 6.5 - NORMALISATION
    let rescaled2 = normalise(&rescaled2);
    save_to_disk(&rescaled2, TEMP_FILE_NAME_RESCALED2);

    // STEP 7 - CONVERT GREYSCALE TO BINARY
    let binary = binary(&rescaled2);
    save_to_disk(&binary, TEMP_FILE_NAME_BINARY);

    // STEP 7 - CONVERT TO STRING
    let mut hash_strings: Vec<String> = Vec::new();

    for i in 0..if *rotations { 4 } else { 1 } {
        hash_strings.push(get_hash_strings(&binary, i));
    }

    hash_strings
}

fn load_from_disk(file_name: &str) -> Result<DynamicImage, ImageError> {
    let load_result = Reader::open(file_name)?.decode();
    if load_result.is_err() {
        logger::log_error(&format!(
            "Failed to load \"{}\": \"{}\".",
            &file_name,
            &load_result.err().unwrap()
        ));
        process::exit(0x0100);
    }
    load_result
}

fn save_to_disk(img: &DynamicImage, file_name: &str) {
    fs::create_dir_all(TEMP_FILE_DIR_PATH);

    let save_result = img.save(format!(
        "{}{}{}",
        TEMP_FILE_DIR_PATH, file_name, TEMP_FILE_EXT
    ));
    if save_result.is_err() {
        logger::log_error(
            &format!(
                "Failed to save \"{}\": {}",
                TEMP_FILE_NAME_LANDSCAPE,
                save_result.err().unwrap()
            )
            .to_string(),
        );
        process::exit(0x0100);
    }
}

fn rescale_image(img: &DynamicImage, w: u32, h: u32) -> DynamicImage {
    logger::log_debug(&format!(
        "Rescaling image from {}*{} to fit {}*{}.",
        img.width(),
        img.height(),
        w,
        h
    ));
    img.thumbnail_exact(w, h)
}

fn discrete_cosine_transform(img: &DynamicImage, chunk_side_length: usize) -> DynamicImage {
    logger::log_debug("Applying Discrete Cosine Transform");

    let mut planner = DctPlanner::<f32>::new();

    let vec_bytes = img.as_bytes();
    let mut bytes = vec![0f32; 0];

    for i in 0..vec_bytes.len() {
        bytes.push(vec_bytes[i] as f32)
    }

    let dct = planner.plan_dct2(chunk_side_length);
    for i in (0..bytes.len()).step_by(chunk_side_length) {
        dct.process_dct2(&mut bytes[i..i + chunk_side_length]);
    }

    let mut transposed = [0f32; (SCALED_SIDE_LENGTH * SCALED_SIDE_LENGTH) as usize];
    transpose::transpose(
        &bytes,
        &mut transposed,
        SCALED_SIDE_LENGTH as usize,
        SCALED_SIDE_LENGTH as usize,
    );

    for i in (0..transposed.len()).step_by(chunk_side_length) {
        dct.process_dct2(&mut transposed[i..i + chunk_side_length]);
    }

    let mut bytes_dct_ed = Vec::<u8>::new();
    for i in 0..transposed.len() {
        bytes_dct_ed.push(transposed[i] as u8)
    }

    let buff = ImageBuffer::from_vec(img.width(), img.height(), bytes_dct_ed).unwrap();
    DynamicImage::ImageLuma8(buff)
}

fn greyscale(img: &DynamicImage) -> DynamicImage {
    logger::log_debug("Converting into greyscale.");
    DynamicImage::from(img.to_luma8())
}

fn normalise(img: &DynamicImage) -> DynamicImage {
    logger::log_debug("Normalising image.");

    let vec_bytes = img.as_bytes();
    let mut bytes = vec![0; vec_bytes.len()];

    let mut max = 0u8;
    let mut min = 0u8;

    for (i, &el) in vec_bytes.iter().enumerate() {
        if el < min {
            min = el;
        } else if el > max {
            max = el;
        }
    }

    for i in 0..vec_bytes.len() {
        bytes[i] = if vec_bytes[i] == 0u8 {
            0
        } else {
            (255f32 * (vec_bytes[i] as f32 / max as f32)) as u8
        }
    }
    for (i, &el) in vec_bytes.iter().enumerate() {
        bytes[i] = if el == 0u8 {
            0
        } else {
            (255f32 * (el as f32 / max as f32)) as u8
        }
    }

    let buff = ImageBuffer::from_vec(img.width(), img.height(), bytes).unwrap();
    DynamicImage::ImageLuma8(buff)
}

fn binary(img: &DynamicImage) -> DynamicImage {
    logger::log_debug("Reducing greyscale to binary.");

    let vec_bytes = img.as_bytes();
    let mut bytes = vec![0; vec_bytes.len()];
    let mut sum = 0u32;

    for (i, &el) in vec_bytes.iter().enumerate() {
        sum = sum + el as u32;
    }

    let avg: u8 = (sum / vec_bytes.len() as u32) as u8;

    for (i, &el) in vec_bytes.iter().enumerate() {
        bytes[i] = if el > avg { 255 } else { 0 }
    }

    let buff = ImageBuffer::from_vec(img.width(), img.height(), bytes).unwrap();
    DynamicImage::ImageLuma8(buff)
}

fn get_hash_strings(img: &DynamicImage, rot_90_deg_count: u8) -> String {
    logger::log_debug(&format!(
        "Producing hash string ({}deg).",
        rot_90_deg_count as u16 * 90u16
    ));

    let mut rotated = img.to_owned();

    if rot_90_deg_count == 1 {
        rotated = img.rotate90();
    } else if rot_90_deg_count == 2 {
        rotated = img.rotate180();
    } else if rot_90_deg_count == 3 {
        rotated = img.rotate270();
    }

    rotated
        .as_bytes()
        .iter()
        .map(|&id| if id == 255 { "1" } else { "0" })
        .collect::<String>()
}

pub fn to_hex(binary_string: &str) -> String {
    let mut hex_string = String::new();
    let mut padded_binary_string = String::new();

    let leading_zeroes_needed = binary_string.len() % 4;

    for i in 0..leading_zeroes_needed {
        padded_binary_string.push_str("0")
    }

    padded_binary_string.push_str(&binary_string);

    for counter in (0..padded_binary_string.len()).step_by(4) {
        match four_bit_word_to_hex(&padded_binary_string[counter..counter + 4]) {
            Some(converted) => hex_string.push_str(converted),
            None => panic!(
                "Failed to convert binary string \"{}\" to hex string.",
                &padded_binary_string[counter..counter + 4]
            ),
        };
    }

    hex_string
}

fn four_bit_word_to_hex(b: &str) -> Option<&str> {
    match b {
        "0000" => Some("0"),
        "0001" => Some("1"),
        "0010" => Some("2"),
        "0011" => Some("3"),
        "0100" => Some("4"),
        "0101" => Some("5"),
        "0110" => Some("6"),
        "0111" => Some("7"),
        "1000" => Some("8"),
        "1001" => Some("9"),
        "1010" => Some("a"),
        "1011" => Some("b"),
        "1100" => Some("c"),
        "1101" => Some("d"),
        "1110" => Some("e"),
        "1111" => Some("f"),
        _ => None,
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//     }
// }
