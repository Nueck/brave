use crate::{CONFIG_PATH, GLOBAL_CONFIG};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

//页面属性枚举
#[derive(Debug, Serialize, Deserialize)]
pub enum PageAttr {
    Static,
    Single,
}

//页面存储地方
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemePosition {
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConf {
    pub public: Vec<Theme>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub img_url: PathBuf,
    pub location: PathBuf,
}

//主题中的设置参数
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: Option<String>,
    pub mode: Option<PageAttr>,
}

impl ThemePosition {}

impl ThemeConf {
    //初始化theme.json的文件
    pub(crate) fn init() {
        let mut config_path = CONFIG_PATH.clone();
        config_path.push("theme.json");

        if !config_path.exists() {
            //获取theme路径
            if let Some(location) = GLOBAL_CONFIG.get_theme().location {
                let mut path_buf = GLOBAL_CONFIG.get_current_path();
                path_buf.push(location);
                //获取公共文件数据
                path_buf.push("public");

                if path_buf.exists() && path_buf.is_dir() {
                    let mut theme_conf = ThemeConf { public: Vec::new() };

                    //只遍历一层
                    let only_traversed_once = 2;
                    find_conf_file_to_theme(path_buf, &mut theme_conf, only_traversed_once);

                    //创建文string数据
                    let str = serde_json::to_string_pretty(&theme_conf)
                        .expect("ThemeConf to Json failure");

                    //将数据存放在json中的
                    let mut file = std::fs::File::create(config_path.as_path())
                        .expect("Could not create theme conf file");
                    file.write_all(str.as_bytes())
                        .expect("Description Failed to write the theme conf file");
                }
            }
        }

        // if config_path.exists() {
        //     let f_config = std::fs::File::open(config_path).expect("Could not open conf file");
        //     let _config: ThemeConf =
        //         serde_json::from_reader(f_config).expect("Could not read conf file");
        // }
    }

    pub fn read_theme_conf() -> Option<ThemeConf> {
        let mut config_path = CONFIG_PATH.clone();
        config_path.push("theme.json");

        if config_path.exists() {
            let f_config =
                std::fs::File::open(config_path).expect("Could not open theme conf file");
            let config: ThemeConf =
                serde_json::from_reader(f_config).expect("Could not read theme conf file");
            return Some(config);
        }
        None
    }
}

//读取所有的conf文件转成theme_conf
fn find_conf_file_to_theme(dir: PathBuf, them_cof: &mut ThemeConf, count: i32) {
    let count = count - 1;
    if count >= 0 {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                find_conf_file_to_theme(path, them_cof, count);
            } else if let Some(ext) = path.extension() {
                if ext == "json" && path.file_name() == Some("conf.json".as_ref()) {
                    match std::fs::File::open(path.clone()) {
                        Ok(f_content) => {
                            let config: Config = serde_json::from_reader(f_content)
                                .expect("Could not read  theme conf file");

                            let img = path.parent().unwrap().join("img.jpg");

                            let theme = if img.exists() {
                                Theme {
                                    name: config.name.unwrap_or("Null".to_string()),
                                    img_url: img,
                                    location: PathBuf::from(path.parent().unwrap()),
                                }
                            } else {
                                Theme {
                                    name: config.name.unwrap_or("Null".to_string()),
                                    img_url: PathBuf::new(),
                                    location: PathBuf::from(path.parent().unwrap()),
                                }
                            };

                            them_cof.public.push(theme);
                        }
                        Err(_) => {}
                    }
                }
            }
        }
    }
}

#[test]
fn test_find() {
    if let Some(location) = GLOBAL_CONFIG.get_theme().location {
        let mut path_buf = GLOBAL_CONFIG.get_current_path();
        path_buf.push(location);
        //获取公共文件数据
        path_buf.push("public");

        if path_buf.exists() && path_buf.is_dir() {
            let mut theme_conf = ThemeConf { public: Vec::new() };
            find_conf_file_to_theme(path_buf, &mut theme_conf, 2);
        }
    }
}
