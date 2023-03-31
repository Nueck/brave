use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataConfig {
    pub data: Option<String>,
    pub data_location: Option<String>,
}

impl DataConfig {
    pub fn get_data_config(&self) -> Self {
        let data = if let Some(data) = &self.data {
            data.to_string()
        } else {
            "data".to_string()
        };

        let data_location = if let Some(data) = &self.data_location {
            data.to_string()
        } else {
            //获取数据路径
            let mut data_dir = dirs::data_dir().unwrap();
            data_dir.push("brave");
            /*判读文件夹存在吗*/
            if !data_dir.exists() {
                fs::create_dir_all(&data_dir).unwrap();
            }

            data_dir.to_str().unwrap().to_string()
        };
        Self {
            data: Some(data.to_owned()),
            data_location: Some(data_location.to_owned()),
        }
    }
}
