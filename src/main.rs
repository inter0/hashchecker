#[cfg(feature = "alloc")]
use base16ct;
use sha2::{Sha256, Digest};
use std::{fs::File, io, path::Path, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str;
    let expected_hash: &str;

    (filename, expected_hash) = parse_arguments(&args);

    //open file
    let path = Path::new(filename);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => {
            print!("Couldn't open file {filename}, because {error}");
            std::process::exit(1);
        }
    };

    let actual_hash = hash_file(file);

    compare_hashes(expected_hash, &actual_hash[..]);
}

fn parse_arguments(args: &Vec<String>) -> (&str, &str) {
    match args.len() {
        3 => {
            let filename = &args[1];
            let hash = &args[2];

            return (filename, hash);
        },
        _ => help(),
    }

    std::process::exit(1);
}

fn help(){
    println!("usage:\n\tfilename <string>:\t\tThe name of the file\n\texpected hash <string>:\t\tThe expected hash of the file");
}

fn hash_file(mut file: File) -> String {
    let mut hasher = Sha256::new();
    match io::copy(&mut file, &mut hasher) {
        Err(_) => {
            print!("Couldn't hash file");
            std::process::exit(1);
        },
        _ => {}
    }
    let hash = hasher.finalize();

    base16ct::lower::encode_string(&hash)
}

fn compare_hashes(expected_hash: &str, actual_hash: &str) {
    println!("Expected Hash:\t{}", expected_hash);
    println!("Actual Hash:\t{}", actual_hash);
    
    if expected_hash == actual_hash {
        println!("The hashes match");
    } else {
        println!("The hashes dont match. Be careful!");
    }
}