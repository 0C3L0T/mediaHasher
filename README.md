# OHASH

This package includes a function used to calculate a hash of a media file based on code found on opensubtitles.org.
source: https://trac.opensubtitles.org/projects/opensubtitles/wiki/HashSourceCodes

## Example:

```rust
use std::fs::File;
let file = File::open("test/breakdance.avi").unwrap();
let fsize = file.metadata().unwrap().len();
let fhash = ohash::create_hash(file, fsize).unwrap();
assert_eq!(fhash, "8e245d9679d31e12");
```
