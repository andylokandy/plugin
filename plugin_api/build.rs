fn main() {
    println!("cargo:rerun-if-changed=*");
    println!(
        "cargo:rustc-env=API_VERSION={}",
        format!("{} {}", env!("CARGO_PKG_VERSION"), get_source_hash())
    );
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    println!("cargo:rustc-env=HOST={}", std::env::var("HOST").unwrap());
    println!(
        "cargo:rustc-env=RUSTC_VERSION={}",
        rustc_version::version_meta().unwrap().short_version_string
    );
}

fn get_source_hash() -> String {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    // Recursively traverse the current directory while automatically
    // filtering out  files and directories according to ignore globs
    // found in files like .ignore and .gitignore
    let files: Result<Vec<_>, ignore::Error> = ignore::Walk::new(".").collect();
    for file in files.unwrap() {
        if file.file_type().unwrap().is_file() {
            let cksm = checksums::hash_file(file.path(), checksums::Algorithm::SHA1);
            cksm.hash(&mut hasher);
        }
    }

    format!("{:x}", hasher.finish())
}
