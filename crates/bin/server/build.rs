use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn main() {
    let manifest_dir_string =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../..");
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../../target/public");
    let output_path = Path::new(&path).join("public");
    println!("cargo:warning={:?}", fs::remove_dir_all(&output_path));
    println!("cargo:warning={:?}", copy_dir_all(input_path, output_path));
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
