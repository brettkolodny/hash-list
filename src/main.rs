use clap::Parser;
use hex;
use json_writer::to_json_string;
use nanoid::nanoid;
use sha3::{Digest, Keccak256};
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short)]
    strings: u32,

    #[arg(short)]
    iterations: u32,

    #[arg(short)]
    output: String,
}

fn main() {
    let args = Args::parse();

    get_hashes(args.strings, args.iterations, args.output);
}

fn get_hashes(num_strings: u32, num_hashes: u32, output: String) {
    let mut hashes: HashMap<String, String> = HashMap::new();

    for _ in 0..(num_strings) {
        let id = nanoid!(10);

        let bytes = id.as_bytes();
        let mut hasher = Keccak256::new();

        hasher.update(bytes);
        let mut result = hasher.finalize();

        for _ in 0..(num_hashes) {
            let mut hasher = Keccak256::new();
            hasher.update(result);
            result = hasher.finalize();
        }

        let hash = format!("0x{}", hex::encode(result));
        hashes.insert(id, hash);
    }

    let json = to_json_string(&hashes);

    fs::write(output, json).expect("Unable to write to file");
}
