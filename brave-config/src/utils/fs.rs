use std::path::{Path, PathBuf};
use std::process::Command;

//生成的链接是默认皮肤的链接
pub fn gen_symlink_default_skin(name: &str) -> bool {
    //TODO:目录位置先写死后期进配置

    let target_dir = Path::new("./themes/public/Default");
    //用于生成用户符号链接
    let mut link_dir = PathBuf::new();
    link_dir.push("./page");
    link_dir.push(name.to_string());

    Command::new("ln")
        .args(&[
            "-s",
            target_dir.to_str().unwrap(),
            "-r",
            link_dir.to_str().unwrap(),
        ])
        .output()
        .expect("REASON")
        .stderr
        .is_empty()
}
