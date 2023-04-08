use serde::{Deserialize, Serialize};

//页面存储地方
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageConfig {
    pub location: Option<String>,
}

impl PageConfig {}
