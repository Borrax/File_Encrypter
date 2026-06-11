use file_encrypter::aes_gcm::{run_application, read_terminal};

fn main() {
    let input_data = match read_terminal(std::io::stdin().lock()) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let _ = run_application(&input_data);
}
