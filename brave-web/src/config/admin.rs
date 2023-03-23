use serde::{Deserialize, Serialize};

use std::path::Path;

//创建.env的结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminState {}

impl AdminState {
    #[allow(unused)]
    pub fn new() -> Self {
        AdminState {}
    }
    /*用于判断是否存在admin的dist文件夹*/
    #[allow(unused)]
    pub fn exist() -> bool {
        Path::new("./admin/dist").exists()
    }
}
