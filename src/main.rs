use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let file_path = args.get(1).expect("Missing file path");
    println!("{}", file_path);
}
