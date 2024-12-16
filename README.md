# Rust Cli File Tool

### Usage 

```
$ ./cmdtool <path> <pattern>
```

### Example 

```
$ ./cmdtool src/main.rs std
File size  2 KB
Last modification time: 1734390451.125861502s
Last access time: 1734390451.135235704s
l: 1, p: 4 | use std::{self, fs::read_to_string};
l: 2, p: 4 | use std::fs;
l: 3, p: 4 | use std::io::{self, Write};
l: 10, p: 10 |     path: std::path::PathBuf,
l: 51, p: 24 | fn get_file_info(path: &std::path::PathBuf){
l: 71, p: 18 |     let content = std::fs::read_to_string(&args.path)
Pattern `std` found in 6 lines.
```
