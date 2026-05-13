use std::env::args;

/// Look up table AES used to replace bytes
///
/// See also [`replace_bytes`]
const SBOX: [u8; 256] = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
];

const RCON: [u8; 11] = [0x00,0x01,0x02,0x04,0x08,0x10,0x20,0x40,0x80,0x1b,0x36];


/// Replaces an array of bytes with their match in a lookup table
///
/// # Arguments:
/// * `state`: An array of bytes to be changed in place
///
/// See also ['SBOX']
fn replace_bytes(state: &mut [u8; 16]) {
    for b in state.iter_mut() {
        *b = SBOX[*b as usize];
    }
}

/// Shifts eatch of the rows of a 4x4 matrix by its index
///
/// # Arguments:
/// * `state`: An array of bytes which is the flattened matrix
///  to have its rows shifted
fn shift_rows(state: &mut [u8; 16]) {
    // b1 b5 b9 b13
    // Row 1 left shift
    state.swap(1, 5); state.swap(5, 9); state.swap(9, 13);
    // Row 2 left shift
    state.swap(2, 10); state.swap(6, 14);
    // Row 3
    state.swap(3, 7); state.swap(3, 11); state.swap(3, 15);
}

/// Multiplies a byte by 2 in Galois Field 2^8
///
/// See also [`mix_columns`]
fn x_gf(b: u8) -> u8 {
    // reducing the byte back to GF if the high bit is 1
    if b & 0x80 != 0 { (b << 1) ^ 0x1b } else { b << 1 }
}

/// Diffusion of the byte matrix columns
///
///# Arguments
///* `s` - A 16 byte array that is essentially the 4x4 byte matrix
fn mix_columns(s: &mut [u8; 16]) {
    for col in 0..4 {
        let row = col * 4;
        // Get the column members
        let (b0, b1, b2, b3) = (s[row], s[row+1], s[row+2], s[row+3]);

        // Multiply each column byte to a fixed matrix
        // in Galois Field
        s[row] = x_gf(b0) ^ x_gf(b1) ^ b1 ^ b2 ^ b3;
        s[row + 1] = b0 ^ x_gf(b1) ^ x_gf(b2) ^ b2 ^ b3;
        s[row + 2] = b0 ^ b1 ^ x_gf(b2) ^ x_gf(b3) ^ b3;
        s[row + 3] = x_gf(b0) ^ b0 ^ b1 ^ b2 ^ x_gf(b3);
    }
}

/// Adding a key in Galois Field by XORing to the state bytes
fn add_round_key(state: &mut [u8; 16], key: &[u8; 16]) {
    for (b, k) in state.iter_mut().zip(key.iter()) {
        *b ^= k;
    }
}

/// Displaying a byte array as a 4x4 matrix
fn display_byte_array(state: &[u8; 16]) {
    let mut grid: [[String; 4]; 4] = Default::default();
    let mut result = String::new();

    let mut col: usize = 0;
    let mut row: usize = 0;
    for (i, val) in state.iter().enumerate() {
        grid[row][col] = val.to_string();

        col = i / grid.len();
        row = i % grid.len();
    }

    for row in grid {
        result.push_str(&row.join(" "));
        result.push('\n');
    }

    println!("{}", result);
}

/// Expanding a 32 bytes key to 240 bytes to be added to the state
///
/// See also [`aes_encrypt_block`]
fn expand_key(key: &[u8; 32]) -> [[u8; 16]; 15] {
    let mut word = [[0u8; 4]; 60];

    #[allow(clippy::needless_range_loop)]
    for i in 0..8 {
        let col = 4*i;
        word[i] = [key[col], key[col+1], key[col+2], key[col+3]];
    }

    for i in 8..60 {
        let mut temp = word[i - 1];

        if i % 8 == 0 {
            temp = [
                SBOX[temp[1] as usize] ^ RCON[i/8],
                SBOX[temp[2] as usize],
                SBOX[temp[3] as usize],
                SBOX[temp[0] as usize]
            ];
        } else if i % 8 == 4 {
            temp = [
                SBOX[temp[0] as usize],
                SBOX[temp[1] as usize],
                SBOX[temp[2] as usize],
                SBOX[temp[3] as usize]
            ];
        }

        word[i] = [
            word[i - 8][0]^temp[0],
            word[i - 8][1]^temp[1],
            word[i - 8][2]^temp[2],
            word[i - 8][3]^temp[3]
        ];
    }

    let mut keys = [[0u8; 16]; 15];

    for round in 0..15 {
        for j in 0..4 {
            keys[round][4*j..4*j+4].copy_from_slice(&word[4*round + j]);
        }
    }

    keys
}

/// Encrypts a block of 16 bytes using the AES algorithm 
///
/// # Arguments:
/// * `block`: A byte array to be encrypted
/// * `keys`: An array of 16 byte chunks each to be used for the block's encryption
fn aes_encrypt_block(block: &[u8; 16], keys: &[[u8; 16]; 15]) -> [u8; 16] {
    let mut state = *block;

    add_round_key(&mut state, &keys[0]);

    // Several rounds as later the key will be 15x16 bytes
    // Using raw numbers for slightly faster encryption
    // instead of being related to the len of keys
    #[allow(clippy::needless_range_loop)]
    for key_idx in 1..14 {
        replace_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &keys[key_idx]);
    }

    // No mix last round
    replace_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &keys[14]);

    state
}

fn aes256_ctr_encrypt(key: &[u8; 32], nonce: &[u8; 12], input: &[u8]) -> Vec<u8> {
    let keys = expand_key(key);
    let mut result = Vec::with_capacity(input.len());
    let mut counter: u32 = 1;

    for chunk in input.chunks(16) {
        let mut block = [0u8; 16];
        block[..12].copy_from_slice(nonce);
        block[12..].copy_from_slice(&counter.to_be_bytes());

        let keystream = aes_encrypt_block(&block, &keys);

        for (i, &byte) in chunk.iter().enumerate() {
            result.push(byte ^ keystream[i]);
        }

        counter = counter.wrapping_add(1);
    }

    result
}

fn main() {
    // let args: Vec<String> = args().collect();
    //
    // let file_path = args.get(1).expect("Missing file path");
    // println!("{}", file_path);

    let input: [u8; 16] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11,
                           0x12, 0x13, 0x14, 0x15];
    
    let mut key = [0u8; 32];
    key[..16].copy_from_slice(&input);
    key[16..].copy_from_slice(&input);

    let keys = expand_key(&key);
    let block = input;

    let encrypted = aes_encrypt_block(&block, &keys);
    display_byte_array(&encrypted);
}
