use std::fs;
use std::fs::File;
mod mediaHash;

fn main() {
    let fname = "breakdance.avi";
    let fsize = fs::metadata(fname).unwrap().len();
    if fsize>mediaHash::HASH_BLK_SIZE {
        let file = File::open(fname).unwrap();
        let fhash = mediaHash::create_hash(file, fsize).unwrap();
        println!("Hash for {} is  {}", fname, fhash);
    }
}