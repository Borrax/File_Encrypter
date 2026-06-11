**file_encrypter > aes_gcm**

# Module: aes_gcm

## Contents

**Structs**

- [`UserInputData`](#userinputdata)

**Enums**

- [`AppError`](#apperror)

**Functions**

- [`aes_gcm_decrypt`](#aes_gcm_decrypt) - Decrypts encrypted text with authentication
- [`aes_gcm_encrypt`](#aes_gcm_encrypt) - Encrypting plain text byte using AES with GCM authenticatioo
- [`decrypt_large_file`](#decrypt_large_file) - Encrypts large files by breaking it into chunks and encrypting each
- [`decrypt_small_file`](#decrypt_small_file) - Decrypts a file using AES256-GCM algorithm
- [`encrypt_large_file`](#encrypt_large_file) - Encrypts large files by breaking it into chunks and encrypting each
- [`encrypt_small_file`](#encrypt_small_file) - Encrypts a file using AES256-GCM algorithm
- [`generate_nonce`](#generate_nonce) - Generates the 12 bytes nonce needed for encryption and decryption
- [`read_terminal`](#read_terminal) - Reads inputs from the terminal and returns them as a tuple
- [`run_application`](#run_application)

---

## file_encrypter::aes_gcm::AppError

*Enum*

**Variants:**
- `MissingInputPath`
- `MissingOutputPath`
- `MissingKey`
- `MetadataError(std::io::Error)`
- `EncryptionError(String)`
- `DecryptionError(String)`

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## file_encrypter::aes_gcm::UserInputData

*Struct*

**Fields:**
- `input_path: Option<String>`
- `output_path: Option<String>`
- `key: Option<[u8; 32]>`
- `should_encrypt: bool`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## file_encrypter::aes_gcm::aes_gcm_decrypt

*Function*

Decrypts encrypted text with authentication

# Arguments:
* `key`: 32 bytes plain key to be used for the encryption
* `nonce`: Random 12 bytes number to be used with the key and the text
* `plain_input`: the raw byte to be encrypted
* `aad`: Additional authentication data (to be used to verify the received information hasn't
  been temepered with)
# `expected_tag`: The expected tag to authenticate with the decryption

See also [`get_authentication_tag`] and [`aes_gcm_encrypt`]

```rust
fn aes_gcm_decrypt(key: &[u8; 32], nonce: &[u8; 12], crypted_text: &[u8], aad: &[u8], expected_tag: &[u8; 16]) -> Option<Vec<u8>>
```



## file_encrypter::aes_gcm::aes_gcm_encrypt

*Function*

Encrypting plain text byte using AES with GCM authenticatioo

# Arguments:
* `key`: 32 bytes plain key to be used for the encryption
* `nonce`: Random 12 bytes number to be used with the key and the text
* `plain_input`: the raw byte to be encrypted
* `aad`: Additional authentication data (to be used to verify the received information hasn't
  been temepered with)

See also [`get_authentication_tag`] and [`aes_ctr_encrypt`]

```rust
fn aes_gcm_encrypt(key: &[u8; 32], nonce: &[u8; 12], plain_input: &[u8], aad: &[u8]) -> (Vec<u8>, [u8; 16])
```



## file_encrypter::aes_gcm::decrypt_large_file

*Function*

Encrypts large files by breaking it into chunks and encrypting each
individual chunk at a time.

# Arguments:
* `input path`: The path to the file to be encrypted
* `output path`: Where the output encrypted file to be generated
* `key`: Raw encryption key
* `aad`: The additional authentication data to encrypt the file with

See also [`encrypt_large_file`]

```rust
fn decrypt_large_file(input_path: &str, output_path: &str, key: &[u8; 32], aad: &[u8]) -> Result<(), AppError>
```



## file_encrypter::aes_gcm::decrypt_small_file

*Function*

Decrypts a file using AES256-GCM algorithm

# Arguments:
* `input path`: The path to the file to be encrypted
* `output path`: Where the output encrypted file to be generated
* `key`: Raw encryption key
* `aad`: The additional authentication data to encrypt the file with

```rust
fn decrypt_small_file(input_path: &str, output_path: &str, key: &[u8; 32], aad: &[u8]) -> Result<(), AppError>
```



## file_encrypter::aes_gcm::encrypt_large_file

*Function*

Encrypts large files by breaking it into chunks and encrypting each
individual chunk at a time.

# Arguments:
* `input path`: The path to the file to be encrypted
* `output path`: Where the output encrypted file to be generated
* `key`: Raw encryption key
* `nonce`: The random generated number to be used for the encryption
* `aad`: The additional authentication data to encrypt the file with

```rust
fn encrypt_large_file(input_path: &str, output_path: &str, key: &[u8; 32], nonce: &[u8; 12], aad: &[u8]) -> Result<(), AppError>
```



## file_encrypter::aes_gcm::encrypt_small_file

*Function*

Encrypts a file using AES256-GCM algorithm

# Arguments:
* `input path`: The path to the file to be encrypted
* `output path`: Where the output encrypted file to be generated
* `key`: Raw encryption key
* `nonce`: The random generated number to be used for the encryption
* `aad`: The additional authentication data to encrypt the file with

```rust
fn encrypt_small_file(input_path: &str, output_path: &str, key: &[u8; 32], nonce: &[u8; 12], aad: &[u8]) -> Result<(), AppError>
```



## file_encrypter::aes_gcm::generate_nonce

*Function*

Generates the 12 bytes nonce needed for encryption and decryption

See also [`aes_gcm_encrypt`] and [`aes_gcm_decrypt`]

```rust
fn generate_nonce() -> [u8; 12]
```



## file_encrypter::aes_gcm::read_terminal

*Function*

Reads inputs from the terminal and returns them as a tuple

If output path is not provided the location where the program is started from
is used

```rust
fn read_terminal<R>(reader: R) -> Result<UserInputData, Box<dyn std::error::Error>>
```



## file_encrypter::aes_gcm::run_application

*Function*

```rust
fn run_application(input_data: &UserInputData) -> Result<(), AppError>
```



