use std::fmt::Write;

use md5::Digest;
use rand::RngExt;

const CHUNK_SIZE: usize = 64;
const _KIB: usize = 1024;

fn main() {
    let mut rng = rand::rng();

    let total_chunks = [
        1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
    ];

    for (index, amount) in total_chunks.iter().enumerate() {
        let name = format!("test_{:02}", index + 1);

        let mut data = vec![0; CHUNK_SIZE * (*amount)];

        rng.fill(data.as_mut_slice());

        hash_data(&name, data);
    }
}

fn hash_data<T>(name: &str, data: T)
where
    T: AsRef<[u8]>,
{
    let data_ref = data.as_ref();
    let mut hashes = String::new();

    writeln!(&mut hashes, "md5:      {}", run_md5(data_ref)).unwrap();
    writeln!(&mut hashes, "sha1:     {}", run_sha1(data_ref)).unwrap();
    writeln!(&mut hashes, "sha2-256: {}", run_sha2_256(data_ref)).unwrap();
    writeln!(&mut hashes, "sha2-384: {}", run_sha2_384(data_ref)).unwrap();
    writeln!(&mut hashes, "sha2-512: {}", run_sha2_512(data_ref)).unwrap();
    writeln!(&mut hashes, "sha3-256: {}", run_sha3_256(data_ref)).unwrap();
    writeln!(&mut hashes, "sha3-384: {}", run_sha3_384(data_ref)).unwrap();
    writeln!(&mut hashes, "sha3-512: {}", run_sha3_512(data_ref)).unwrap();
    writeln!(&mut hashes, "blake3:   {}", run_blake3(data_ref)).unwrap();

    std::fs::write(format!("./input/{name}.data"), data_ref)
        .expect("failed to create input data file");
    std::fs::write(format!("./input/{name}.hashes"), hashes)
        .expect("failed to create input hashes file");
}

fn run_md5(data: &[u8]) -> String {
    let mut hasher = md5::Md5::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().to_vec())
}

fn run_sha1(data: &[u8]) -> String {
    let mut hasher = sha1::Sha1::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().to_vec())
}

fn run_sha2_256(data: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().to_vec())
}

fn run_sha2_384(data: &[u8]) -> String {
    let mut hasher = sha2::Sha384::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().to_vec())
}

fn run_sha2_512(data: &[u8]) -> String {
    let mut hasher = sha2::Sha512::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().to_vec())
}

fn run_sha3_256(data: &[u8]) -> String {
    let mut hasher = sha3::Sha3_256::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().to_vec())
}

fn run_sha3_384(data: &[u8]) -> String {
    let mut hasher = sha3::Sha3_384::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().to_vec())
}

fn run_sha3_512(data: &[u8]) -> String {
    let mut hasher = sha3::Sha3_512::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().to_vec())
}

fn run_blake3(data: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();

    hasher.update(data);

    to_hex_str(hasher.finalize().as_bytes().to_vec())
}

pub const HEX_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

pub fn to_hex_str<T>(bytes: T) -> String
where
    T: AsRef<[u8]>,
{
    let slice = bytes.as_ref();
    let mut rtn = String::with_capacity(slice.len() * 2);

    for byte in slice {
        let upper = ((byte & 0xF0) >> 4) as usize;
        let lower = (byte & 0x0F) as usize;

        rtn.push(HEX_CHARS[upper]);
        rtn.push(HEX_CHARS[lower]);
    }

    rtn
}
