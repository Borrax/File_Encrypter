# File Encrypter

![rust logo](https://rust-lang.org/logos/rust-logo-512x512.png)

### General Information:
A CLI tool that encrypts/decrypts a file or multiple files by a given path using the AES-265-GCM algorithm.

If the input file is larger in size (for testing >4MB) it would encrypt/decrypt it one chunk at the time, otherwise it would load it awhole and process it.
The algorithm is using randomly generated number (nonce) and additional authenticated data (AAD, which for now is a static string) to be used
together with the encryption.

You can also check the [high-level flow diagram](#high-level-flow-diagram)

### Usage:
```bash
cargo run -- <-k <encryption_key> -i <input_file_path> [-o <output_file_path>]
```
or if you have the built target:
```bash
<target_name> -k <encryption_key> -i <input_file_path> [-o <output_file_path>]
```

By default it would encrypt a file. If you need to decrypt it use the ```-d``` flag.

For general help use the ```-h``` flag.

### Testing
Run integration tests:
```bash
cargo test --test integration_tests
```
Run unit tests:
```bash
cargo test --lib aes_gcm::unit_tests::
```

### Viewing documentation
```bash
cargo doc --open
```

### High-level flow diagram

![High-level_flow_diagram](./docs/app_flow_diagram.svg)
