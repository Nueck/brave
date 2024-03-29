use crate::utils::error::AuthError;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Validation,
};
use once_cell::sync::{Lazy, OnceCell};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};

//全局的变量
pub static GLOB_JOT: Lazy<Jot> = Lazy::new(|| Jot::new());
pub static GLOB_JWT_CONFIG: OnceCell<JWTConfig> = OnceCell::new();

//JWT的配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JWTConfig {
    pub exp_time: Option<u64>,
    pub sub: Option<String>,
    pub ref_time: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub aud: String,
    pub sub: String,
    pub exp: u64,
    pub auth: String,
    pub data: Option<UserData>,
    pub refresh: bool,
}

//用户传来的数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub code: String,
    pub email: String,
}

//用于存放需要验证的信息
pub struct TokenMsg {
    pub token: String,
    pub ip: String,
}

/*用于存放token解析后的数据*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDataInfo {
    pub id: i32,
    pub aud: String,
    pub auth: String,
    pub refresh: bool,
}

//用于存放的编码和解码密钥
pub struct Jot {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Jot {
    pub fn new() -> Self {
        let doc = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
        let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

        let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
        let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

        Self {
            encoding_key,
            decoding_key,
        }
    }

    pub fn generate_token(&self, claims: &Claims) -> String {
        //加密
        encode(
            &jsonwebtoken::Header::new(Algorithm::EdDSA),
            &claims,
            &self.encoding_key,
        )
        .unwrap()
    }

    pub fn validation_token(&self, token_msg: &TokenMsg) -> Result<UserDataInfo, AuthError> {
        let validation = Validation::new(Algorithm::EdDSA);
        let token_data = match decode::<Claims>(&token_msg.token, &self.decoding_key, &validation) {
            Ok(c) => c,
            Err(err) => {
                return match *err.kind() {
                    ErrorKind::InvalidToken => {
                        log::warn!("Token is invalid - IP: {}", &token_msg.ip);
                        Err(AuthError::VerifyError)
                    }
                    ErrorKind::InvalidIssuer => {
                        log::warn!("Issuer is invalid - IP: {}", &token_msg.ip);
                        Err(AuthError::VerifyError)
                    }
                    _ => {
                        log::warn!(
                            "The token authentication is faulty. Procedure - Ip: {} -- {}",
                            &token_msg.ip,   //这里打印以下出错ip以便于调查问题
                            err.to_string()  //打印错误
                        );
                        Err(AuthError::VerifyError)
                    }
                };
            }
        };
        //判断时间是否过期
        if token_data.claims.exp < get_current_timestamp() {
            return Err(AuthError::ExpirationError);
        } else if token_data.claims.sub != JWTConfig::global().get_sub() {
            return Err(AuthError::VerifyError);
        }
        //验证成功
        Ok(UserDataInfo {
            id: token_data.claims.id,
            aud: token_data.claims.aud,
            auth: token_data.claims.auth,
            refresh: token_data.claims.refresh,
        })
    }

    /*用于登陆验证码的的验证*/
    pub fn validation_to_claim(&self, token_msg: &str) -> Result<Claims, AuthError> {
        let validation = Validation::new(Algorithm::EdDSA);
        match decode::<Claims>(&token_msg, &self.decoding_key, &validation) {
            Ok(data) => {
                //判断时间是否正确
                if data.claims.exp < get_current_timestamp() {
                    Err(AuthError::VerifyError)
                } else {
                    Ok(data.claims)
                }
            }
            Err(_) => Err(AuthError::VerifyError),
        }
    }
}

//JWTConfig配置
impl JWTConfig {
    pub fn global() -> &'static JWTConfig {
        GLOB_JWT_CONFIG
            .get()
            .expect("jwt config is not initialized")
    }

    pub fn new(jwt_config: JWTConfig) {
        GLOB_JWT_CONFIG
            .set(jwt_config)
            .expect("JWT config Initialization failure")
    }

    pub fn get_exp_time(&self) -> u64 {
        match &self.exp_time {
            None => 1000, //默认值
            Some(num) => num.to_owned(),
        }
    }

    pub fn get_ref_time(&self) -> u64 {
        match &self.ref_time {
            None => 1000, //默认值
            Some(num) => num.to_owned(),
        }
    }

    pub fn get_sub(&self) -> String {
        match &self.sub {
            None => "brave".to_string(),
            Some(s) => s.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let jwt = JWTConfig {
            exp_time: Some(1000),
            sub: Some("blog".to_string()),
            ref_time: Some(1000),
        };

        JWTConfig::new(jwt);

        let jot = Jot::new();
        let claims = Claims {
            id: 0,
            aud: "blog".to_string(),
            sub: "blog".to_string(),
            exp: get_current_timestamp() + JWTConfig::global().get_exp_time(),
            auth: "super".to_string(),
            data: None,
            refresh: false,
        };

        let token = jot.generate_token(&claims);

        let token_msg = TokenMsg {
            token,
            ip: "".to_string(),
        };

        jot.validation_token(&token_msg).unwrap();
    }
}
