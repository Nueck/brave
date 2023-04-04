use serde::{Deserialize, Serialize};

///用户表数据
#[derive(Deserialize, Serialize)]
pub(crate) struct UserTableData {
    pub(crate) user_id: i32,
    pub(crate) user_name: String,
    pub(crate) authority: String,
    pub(crate) email: String,
    pub(crate) user_status: i16,
}
