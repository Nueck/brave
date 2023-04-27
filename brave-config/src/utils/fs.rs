use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

//生成的链接是默认皮肤的链接
pub fn gen_default_skin_page(name: &str) -> bool {
    let src_dir = Path::new("./themes/public/Default");
    let mut dest_dir = PathBuf::new();
    dest_dir.push("./page");
    dest_dir.push(name.to_string());

    if let Err(e) = copy_directory_recursively(src_dir, dest_dir) {
        log::error!("copy dir err:  {}", e);
        false
    } else {
        true
    }
}

fn copy_directory_recursively<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dest: Q,
) -> std::io::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    if !src.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Source path is not a directory",
        ));
    }

    fs::create_dir_all(&dest)?;

    for entry in WalkDir::new(src) {
        let entry = entry?;
        let file_type = entry.file_type();
        let src_path = entry.path();
        let dest_path = dest.join(src_path.strip_prefix(src).unwrap());

        if file_type.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else if file_type.is_file() {
            fs::copy(&src_path, &dest_path)?;
        } else {
            log::error!("Skipping {:?}", src_path);
        }
    }
    Ok(())
}

#[test]
fn test_gen_default_skin_page() {
    gen_default_skin_page("Default");
}
