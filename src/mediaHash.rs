use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader};
use std::mem;

pub const HASH_BLK_SIZE: u64 = 65536;

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

