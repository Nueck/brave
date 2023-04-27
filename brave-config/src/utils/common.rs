use crate::{GLOBAL_CONFIG, GLOBAL_DATA, GLOBAL_START_TIME};
use actix_rt::time;
use jsonwebtoken::get_current_timestamp;
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

//验证码验证
pub static GLOBAL_CODE: Lazy<Mutex<HashMap<u32, u64>>> = Lazy::new(|| {
    let hash = HashMap::<u32, u64>::new();
    Mutex::new(hash)
});

//判断是否是无效的用户名
pub fn is_invalid_user_name(user: &str) -> bool {
    user == GLOBAL_CONFIG.interface.api_scope
        || user == GLOBAL_CONFIG.interface.admin_scope
        || user == GLOBAL_CONFIG.interface.blog_scope
        || user == GLOBAL_DATA.get_data_config().data.unwrap()
        || user == "css"
        || user == "js"
        || user == "img"
        || user == "fonts"
}

/*判断是否是邮箱地址*/
pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})$").unwrap();
    re.is_match(email)
}

/*判断是否是outlook邮箱*/
pub fn is_outlook_email(email: &str) -> bool {
    let email_parts: Vec<&str> = email.split('@').collect();
    if email_parts.len() != 2 {
        return false;
    }
    let domain = email_parts[1].to_lowercase();
    domain == "outlook.com"
}

///生成随机数
pub fn generation_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    let code = rng.gen_range(100000..=999999);
    if let Some(_) = GLOBAL_CODE.lock().unwrap().get(&code) {
        generation_random_number()
    } else {
        code
    }
}

///判断文件路径是否是html
pub fn is_html_path(path: &str) -> bool {
    path.ends_with(".html")
}

///判断一个string是否一个网站
pub fn is_web_path(path: &str) -> bool {
    let re = Regex::new(r"(https?)://[-A-Za-z0-9+&@#/%?=~_|!:,.;]+[-A-Za-z0-9+&@#/%=~_|]").unwrap();
    re.is_match(path)
}

//获取当前系统的启动时间
pub fn get_system_uptime() -> u64 {
    GLOBAL_START_TIME.get().unwrap().to_owned()
}

//实现定时任务
pub async fn async_code_process() {
    actix_rt::spawn(async {
        //每一分钟开始清除
        let mut interval = time::interval(Duration::from_secs(60));
        //开始循环任务
        loop {
            interval.tick().await;
            if GLOBAL_CODE.lock().unwrap().is_empty() {
                continue;
            }
            let current_time = get_current_timestamp();
            for (key, value) in GLOBAL_CODE.lock().unwrap().iter() {
                if value < &current_time {
                    GLOBAL_CODE.lock().unwrap().remove(key);
                }
            }
        }
    });
}

#[cfg(test)]
mod common_tests {
    use super::*;

    #[test]
    fn common_test1() {
        println!("{}", generation_random_number())
    }

    #[test]
    fn common_test2() {
        let str = "http://www.qweqwe.cn";
        println!("{}", is_web_path(&str))
    }
}
