use std::env::args;
use file_encrypter::aes_gcm::{aes_gcm_decrypt, aes_gcm_encrypt, generate_nonce};

fn main() {
    // let args: Vec<String> = args().collect();
    //
    // let file_path = args.get(1).expect("Missing file path");
    // println!("{}", file_path);

    // let input: [u8; 16] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11,
    //                        0x12, 0x13, 0x14, 0x15];
    
    let text_byte = b"Hello there";
    let key = b"test keytest ketest ketest keyyy";
    let nonce = generate_nonce();
    let aad = b"my_checksum";
    println!("Raw text: {:?}", text_byte);
    println!("Nonce: {:?}", nonce);

    let (crypted_text, tag) = aes_gcm_encrypt(key, &nonce, text_byte, aad);

    println!("Crypted text: {:?}", crypted_text);
    println!("Tag: {:?}", tag);

    let decrypted_bytes = aes_gcm_decrypt(key, &nonce, &crypted_text, aad, &tag);

    println!("Decrypted bytes: {:?}", decrypted_bytes);
}
