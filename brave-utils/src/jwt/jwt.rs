use crate::error::AuthError;
use crate::jwt::config::JWTConfig;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Validation,
};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

//用于存放需要验证的信息
pub struct TokenMsg {
    pub token: String,
    pub ip: String,
}

//用于存放的编码和解码密钥
pub struct Jot {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Jot {
    pub fn new() -> Jot {
        let doc = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
        let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

        let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
        let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

        Jot {
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

    pub fn validation_token(&self, token_msg: &TokenMsg) -> Result<bool, AuthError> {
        let validation = Validation::new(Algorithm::EdDSA);
        let token_data = match decode::<Claims>(&token_msg.token, &self.decoding_key, &validation) {
            Ok(c) => c,
            Err(err) => {
                return match *err.kind() {
                    ErrorKind::InvalidToken => {
                        log::error!("Token is invalid - IP: {}", &token_msg.ip);
                        Err(AuthError::VerifyError)
                    }
                    ErrorKind::InvalidIssuer => {
                        log::error!("Issuer is invalid - IP: {}", &token_msg.ip);
                        Err(AuthError::VerifyError)
                    }
                    _ => {
                        log::error!(
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
        Ok(true)
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
        };

        JWTConfig::new(jwt);

        let jot = Jot::new();
        let claims = Claims {
            sub: "blog".to_string(),
            exp: get_current_timestamp() + JWTConfig::global().get_exp_time(),
        };

        let token = jot.generate_token(&claims);

        let token_msg = TokenMsg {
            token,
            ip: "".to_string(),
        };

        jot.validation_token(&token_msg).unwrap();
    }
}
