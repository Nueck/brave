use crate::{CONFIG_PATH, GLOBAL_CONFIG, GLOB_INIT};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::sync::{Mutex, MutexGuard};

/*初始化的状态 */
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InitStatus {
    pub is_init: bool,
    pub username: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub registrants: i64,
    pub able_register: bool,
}

impl InitStatus {
    pub fn global() -> MutexGuard<'static, InitStatus> {
        GLOB_INIT
            .get()
            .expect("InitStatus config is not initialized")
            .lock()
            .expect("Lock acquisition failure")
    }

    pub fn new(init: Option<InitStatus>) {
        let mut config_path = CONFIG_PATH.clone();

        config_path.push("config");
        //判断文件是否存在 .config/brave/config
        if config_path.exists() {
            let f_config = std::fs::File::open(config_path).expect("Could not open conf file");
            let config: InitStatus =
                serde_json::from_reader(f_config).expect("Could not read conf file");

            GLOB_INIT
                .set(Mutex::from(config))
                .expect("InitStatus Initialization failure")
        } else {
            /*判断是否为空*/
            let info = match init {
                None => InitStatus {
                    is_init: false,
                    username: None,
                    email: None,
                    address: None,
                    registrants: GLOBAL_CONFIG.get_registrants(),
                    able_register: GLOBAL_CONFIG.get_able_register(),
                },
                Some(init) => init,
            };

            /*需要创建文件并且将数据存在配置文件*/
            let json = serde_json::to_string_pretty(&info).expect("InitStatus to Json failure");
            /*将文件保存在配置文件中*/
            let mut file = File::create(config_path.as_path()).expect("Could not open conf file");
            file.write_all(json.as_bytes())
                .expect("Description Failed to write the configuration file");

            GLOB_INIT
                .set(Mutex::from(info))
                .expect("InitStatus config Initialization failure")
        }
    }

    /*用于初始化设置初始化状态的*/
    pub fn set(init: InitStatus) {
        let mut init_status = GLOB_INIT
            .get()
            .expect("Could not get InitStatus")
            .lock()
            .unwrap();
        init_status.is_init = init.is_init;
        init_status.email = init.email;
        init_status.address = init.address;
        init_status.username = init.username;
        init_status.registrants = init.registrants;
        init_status.able_register = init.able_register;
        write_config_file(&init_status);
    }

    pub fn set_registrants(registrants: i64) {
        let mut init_status = GLOB_INIT
            .get()
            .expect("Could not get InitStatus")
            .lock()
            .unwrap();
        init_status.registrants = registrants;
        write_config_file(&init_status);
    }

    pub fn set_able_register(able_register: bool) {
        let mut init_status = GLOB_INIT
            .get()
            .expect("Could not get InitStatus")
            .lock()
            .unwrap();
        init_status.able_register = able_register;
        write_config_file(&init_status);
    }
}

fn write_config_file(init: &InitStatus) {
    //获取文件位置信息
    let mut config_path = CONFIG_PATH.clone();

    config_path.push("config");

    //将文件保存在配置文件中
    let json = serde_json::to_string_pretty(init).expect("InitStatus to Json failure");
    let mut file = File::create(config_path.as_path()).expect("Could not open conf file");
    file.write(json.as_bytes())
        .expect("Description Failed to write the configuration file");
}
