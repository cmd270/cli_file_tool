> This project was made only for learning Rust lang. No Payloads here. :)

# Cli File Tool

### Usage 
```
$ ./cmdtool <path> <pattern>
```
### Example 
```
$ ./cmdtool src/main.rs path 
File name: main.rs 🍣
Size: 2356 bytes
Creation time: Tue, 17 Dec 2024 01:25:44 +0000
Last modification time: Tue, 17 Dec 2024 04:37:25 +0000
Last access time: Tue, 17 Dec 2024 04:37:25 +0000
ln: 21, col: 4, 15 | `    path: std::path::PathBuf,`
ln: 25, col: 17, 29 | `fn get_file_info(path: &std::path::PathBuf){`
ln: 26, col: 32 | `    let metadata = fs::metadata(path).unwrap();`
ln: 46, col: 35 | `    println!("File name: {} 🍣", path.file_name()`
ln: 86, col: 48 | `    let content = std::fs::read_to_string(&args.path)`
ln: 87, col: 68 | `        .with_context(|| format!("could not read file `{:?}`", args.path))?;`
ln: 89, col: 24 | `    get_file_info(&args.path);`
Pattern `path` found in 7 lines.
```