use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("Running build script...");

    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OUT_DIR: {}", out_dir);

    let target_dir = Path::new(&out_dir).join("../../../release");
    println!("Target directory: {:?}", target_dir);

    let lib_name = "crate_shared";
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let new_name = format!("lib{}_v{}.so", lib_name, version);

    let old_path = target_dir.join(format!("lib{}.so", lib_name));
    let new_path = target_dir.join(new_name);

    println!("Old path: {:?}", old_path);
    println!("New path: {:?}", new_path);

    if old_path.exists() {
        fs::rename(&old_path, &new_path).unwrap();
        println!("Renamed {:?} to {:?}", old_path, new_path);
    } else {
        println!("{:?} does not exist", old_path);
    }
}
