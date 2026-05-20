use std::env;
use file_encrypter::aes_gcm::{aes_gcm_decrypt, aes_gcm_encrypt, encrypt_file, generate_nonce};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut output_path: String = env::current_dir().unwrap().display().to_string();
    let input_path = args.get(1).expect("Missing file path");

    if args.len() > 2 {
        output_path = args.get(2).unwrap().to_string();
    }

    let key = b"test keytest ketest ketest keyyy";
    let nonce = generate_nonce();
    let aad = b"my_checksum";

    let _ = encrypt_file(input_path, &output_path, key, &nonce, aad);

    // let text_byte = b"Hello there";
}
