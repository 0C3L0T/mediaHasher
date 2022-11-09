//! # OHASH
//! 
//! ## Example:
//! 
//! ```
//! use std::fs::File;
//! let file = File::open("test/breakdance.avi").unwrap();
//! let fsize = file.metadata().unwrap().len();
//! let fhash = ohash::create_hash(file, fsize).unwrap();
//! assert_eq!(fhash, "8e245d9679d31e12");
//! ```

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader};
use std::mem;

/// Defines the size of the blocks used to create the hash
pub const HASH_BLK_SIZE: u64 = 65536;

/// Create a hash from a file given a File struct and the size of the file
pub fn create_hash(file: File, fsize: u64) -> Result<String, std::io::Error> {

    let mut buf = [0u8; 8];
    let mut word: u64;

    let mut hash_val: u64 = fsize;  // seed hash with file size

    let iterations = HASH_BLK_SIZE /  8;

    let mut reader = BufReader::with_capacity(HASH_BLK_SIZE as usize, file);

    for _ in 0..iterations {
        reader.read(&mut buf)?;
        unsafe { word = mem::transmute(buf); };
        hash_val = hash_val.wrapping_add(word);
    }

    reader.seek(SeekFrom::Start(fsize - HASH_BLK_SIZE))?;

    for _ in 0..iterations {
        reader.read(&mut buf)?;
        unsafe { word = mem::transmute( buf); };
        hash_val = hash_val.wrapping_add(word);
    }

    let hash_string = format!("{:01$x}", hash_val, 16);

    Ok(hash_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hash() {
        let files = ["test/breakdance.avi"];
        let hashes = ["8e245d9679d31e12"];

        for (i, file) in files.iter().enumerate() {
            let f = File::open(file).unwrap();
            let fsize = f.metadata().unwrap().len();
            let fhash = create_hash(f, fsize).unwrap();
            assert_eq!(fhash, hashes[i]);
        }
    }
}
