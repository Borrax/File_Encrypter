use std::{fs, io::{BufReader, Read}};
use file_encrypter::aes_gcm::{run_application, UserInputData};

#[test]
fn test_application_simple_file() {
    let input_path = "./tests/test_file.txt";
    let output_path_enc = "./tests/encrypted_test_file";
    let output_path_dec = "./tests/decrypted_test_file";

    if fs::exists(output_path_enc).unwrap() {
        fs::remove_file(output_path_enc).unwrap();
    }

    if fs::exists(output_path_dec).unwrap() {
        fs::remove_file(output_path_dec).unwrap();
    }

    let key = b"12345678901234567890123456789012";

    let mut input_data = UserInputData {
        input_path: Some(input_path.to_string()),
        output_path: Some(output_path_enc.to_string()),
        key: Some(*key),
        ..Default::default()
    };

    let app_result = run_application(&input_data);
    assert!(app_result.is_ok());

    assert!(fs::exists(output_path_enc).unwrap(), "Encrypted file does not exist");

    input_data.input_path = Some(output_path_enc.to_string());
    input_data.output_path = Some(output_path_dec.to_string());
    input_data.should_encrypt = false;

    let app_result = run_application(&input_data);
    assert!(app_result.is_ok());

    assert!(fs::exists(output_path_dec).unwrap(), "Decrypted file does not exist");

    let decrypted_file = fs::read(output_path_dec).unwrap();
    let original_file = fs::read(input_path).unwrap();

    assert_eq!(decrypted_file, original_file, "Decrypted and original file are not equal");
}

#[test]
fn test_application_large_file() {
    let input_path = "./tests/video_file.mp4";
    let output_path_enc = "./tests/encrypted_large_file";
    let output_path_dec = "./tests/decrypted_large_file";
    const CHUNK_SIZE: usize = 64;

    if fs::exists(output_path_enc).unwrap() {
        fs::remove_file(output_path_enc).unwrap();
    }

    if fs::exists(output_path_dec).unwrap() {
        fs::remove_file(output_path_dec).unwrap();
    }

    let key = b"12345678901234567890123456789012";

    let mut input_data = UserInputData {
        input_path: Some(input_path.to_string()),
        output_path: Some(output_path_enc.to_string()),
        key: Some(*key),
        ..Default::default()
    };

    let app_result = run_application(&input_data);
    assert!(app_result.is_ok());

    assert!(fs::exists(output_path_enc).unwrap(), "Encrypted file does not exist");
    //
    input_data.input_path = Some(output_path_enc.to_string());
    input_data.output_path = Some(output_path_dec.to_string());
    input_data.should_encrypt = false;

    let app_result = run_application(&input_data);
    assert!(app_result.is_ok());

    assert!(fs::exists(output_path_dec).unwrap(), "Decrypted file does not exist");

    let file_size_dec = fs::metadata(output_path_dec).unwrap().len();
    let file_size_orig = fs::metadata(input_path).unwrap().len();

    assert_eq!(file_size_dec, file_size_orig);

    let input_file_handle = fs::File::open(input_path).unwrap();
    let output_file_handle = fs::File::open(output_path_dec).unwrap();
    let mut dec_reader = BufReader::new(output_file_handle);
    let mut orig_reader = BufReader::new(input_file_handle);
    let mut dec_data_buf = [0u8; CHUNK_SIZE];
    let mut orig_data_buf = [0u8; CHUNK_SIZE];

    loop {
        match dec_reader.read_exact(&mut dec_data_buf) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => panic!("{:?}", e)
        }

        let _ = orig_reader.read_exact(&mut orig_data_buf);

        assert_eq!(dec_data_buf, orig_data_buf, "Decrypted and original data are not equal!");
    }
}
