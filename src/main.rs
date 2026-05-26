use file_encrypter::aes_gcm::{run_application, read_terminal};

fn main() {
    let input_data = read_terminal(std::io::stdin().lock());

    run_application(&input_data);
}
