use file_encrypter::aes_gcm::{aes_gcm_decrypt, aes_gcm_encrypt, generate_nonce};

#[test]
fn test_encrypt_decrypt_simple() {
    let text_bytes = b"Hello there";
    let key = b"test keytest ketest ketest keyyy";
    let nonce = generate_nonce();
    let aad = b"my_checksum";

    let (crypted_text, tag) = aes_gcm_encrypt(key, &nonce, text_bytes, aad);
    let decrypted_bytes = aes_gcm_decrypt(key, &nonce, &crypted_text, aad, &tag);

    assert_eq!(decrypted_bytes.unwrap(), text_bytes);
}
