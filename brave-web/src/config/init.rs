use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::sync::Mutex;

/*初始化的用户信息*/
#[derive(Deserialize)]
pub struct InitInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: String,
}

//全局的一个只能变化一次的
pub static GLOB_INIT_DATA: OnceCell<Mutex<InitInfo>> = OnceCell::new();
