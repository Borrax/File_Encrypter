use file_encrypter::aes_gcm::{encrypt_file, generate_nonce, read_terminal};

fn main() {
    let (input_path, output_path) = read_terminal();
    let key = b"test keytest ketest ketest keyyy";
    let nonce = generate_nonce();
    let aad = b"my_checksum";

    let _ = encrypt_file(&input_path, &output_path, key, &nonce, aad);

    // let text_byte = b"Hello there";
}
