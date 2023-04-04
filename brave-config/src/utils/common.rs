use crate::GLOBAL_CONFIG;
use rand::Rng;
use regex::Regex;

//判断是否是无效的用户名
pub fn is_invalid_user_name(user: &str) -> bool {
    user == GLOBAL_CONFIG.interface.api_scope
        || user == GLOBAL_CONFIG.interface.admin_scope
        || user == GLOBAL_CONFIG.interface.blog_scope
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

/*生成随机数*/
pub fn generation_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(100000..=999999)
}

/*判断文件路径是否是html*/
pub fn is_html_path(path: &str) -> bool {
    path.ends_with(".html")
}

/*判断一个string是否一个网站*/
pub fn is_web_path(path: &str) -> bool {
    let re = Regex::new(r"(https?)://[-A-Za-z0-9+&@#/%?=~_|!:,.;]+[-A-Za-z0-9+&@#/%=~_|]").unwrap();
    re.is_match(path)
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
