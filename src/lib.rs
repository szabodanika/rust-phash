use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jfloat, jstring};

mod hashing;
mod logger;

#[no_mangle]
pub extern "system" fn Java_uk_ac_uws_danielszabo_automodera_common_service_hashing_RustPHash_getSimilarityToFileWithHash(
	env: JNIEnv,
	class: JClass,
	file_name: JString,
	reference_hash: JString,
) -> jfloat {
	let file_name: String = env.get_string(file_name).expect("Couldn't get java string \"file_name\" !").into();

	let reference_hash: String = env.get_string(reference_hash).expect("Couldn't get java string \"reference_hash\" !").into();

	// calculate 4 hashes for given file
	let hashes1 = hashing::hash(&file_name, &true).clone();

	// put parameter hash in a vector
	let hashes2 = vec![reference_hash];

	// calculate similarity
	let similarity = hashing::similarity(&hashes1, &hashes2);

	// return similarity score as java float
	jfloat::from(similarity)
}

#[no_mangle]
pub extern "system" fn Java_uk_ac_uws_danielszabo_automodera_common_service_hashing_RustPHash_getHashForFile(
	env: JNIEnv,
	class: JClass,
	file_name: JString,
) -> jstring {
	let file_name: String = env.get_string(file_name).expect("Couldn't get java string \"file_name\" !").into();

	// calculate 4 hashes for given file
	let hashes = hashing::hash(&file_name, &true).clone();
	let hash_string = hashing::to_hex(&hashes[0]);
	let output = env.new_string(&hash_string).expect("Couldn't create java string!");

	output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_uk_ac_uws_danielszabo_automodera_common_service_hashing_RustPHash_getSimilarityScore(
	env: JNIEnv,
	class: JClass,
	hash1: JString,
	hash2: JString,
) -> jfloat {
	let hash1: String = env.get_string(hash1).expect("Couldn't get java string \"hash1\" !").into();
	let hash2: String = env.get_string(hash2).expect("Couldn't get java string \"hash2\" !").into();

	let sim_score = hashing::hash_similarity(&hash1, &hash2);

	jfloat::from(sim_score)
}
