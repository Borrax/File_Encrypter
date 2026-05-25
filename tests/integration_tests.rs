use std::fs;
use file_encrypter::aes_gcm::{run_application, UserInputData};

#[test]
fn test_application_simple_file() {
    let input_path = "./tests/test_file.txt";
    let output_path = "./tests/encrypted_test_file";

    if fs::exists(output_path).unwrap() {
        fs::remove_file(output_path).unwrap();
    }

    let key = b"12345678901234567890123456789012";

    let mut input_data = UserInputData::default();
    input_data.input_path = Some(input_path.to_string());
    input_data.output_path = output_path.to_string();
    input_data.key = Some(key.clone());

    run_application(&input_data);

    assert!(fs::exists(output_path).unwrap(), "Encrypted file does not exist")
}
