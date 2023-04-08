use serde::{Deserialize, Serialize};

//页面存储地方
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemePosition {
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeConf {}

impl ThemePosition {}

impl ThemeConf {
    //初始化theme.json的文件
}
