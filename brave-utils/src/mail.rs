use crate::common::is_outlook_email;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

//mail的配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailConfig {
    pub mine_email: String,
    pub smtp_server: String,
    pub password: String,
    pub port: u16,
}

impl MailConfig {
    pub async fn sendmail(&self, target_email: &str, code: &str) -> bool {
        let email = Message::builder()
            .from((&self.mine_email.clone()).parse().unwrap())
            .to(target_email.parse().unwrap())
            .subject("Brave验证码")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(
                "你的验证码是".to_owned() + &code + "  有效时间5分钟",
            ))
            .unwrap();

        let passwd = &self.password;
        let min_email = &self.mine_email;
        let port = &self.port;
        let creds = Credentials::new(min_email.to_owned(), passwd.to_owned());

        /*判断是都是outlook邮箱*/
        let mailer = if is_outlook_email(min_email) {
            SmtpTransport::starttls_relay(&self.smtp_server)
        } else {
            SmtpTransport::relay(&self.smtp_server)
        }
        .unwrap()
        .credentials(creds)
        .port(port.to_owned())
        .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => {
                log::info!("Email sent successfully!");
                true
            }
            Err(e) => {
                log::error!("Could not send email: {e:?}");
                false
            }
        }
    }
}

#[cfg(test)]
mod mail_tests {
    use super::*;

    #[test]
    fn mail_test1() {
        let mail = MailConfig {
            mine_email: "brave-rust@outlook.com".to_string(),
            smtp_server: "smtp.office365.com".to_string(),
            password: "--".to_string(),
            port: 587,
        };
        mail.sendmail("nueck@outlook.com".parse().unwrap(), "12345");
    }
}
