use std::future::{ready, Ready};
use std::task::{Context, Poll};

use crate::config::GLOBAL_YAML_CONFIG;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use brave_utils::error::AuthError;
use brave_utils::jwt::jwt::TokenMsg;
use brave_utils::jwt::jwt::GLOB_JOT;
use futures_util::future::LocalBoxFuture;

pub struct JWTAuth;

impl<S, B> Transform<S, ServiceRequest> for JWTAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JWTAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTAuthMiddleware { service }))
    }
}

pub struct JWTAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JWTAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        //获取ip
        let addr = req.peer_addr().unwrap();
        let ip = addr.ip().to_string();

        return if is_need_verification(req.path()) {
            //首先判断有没有认证
            return match req.headers().get(header::AUTHORIZATION) {
                None => Box::pin(async { Err(ErrorUnauthorized("No Authentication")) }),
                Some(header_value) => {
                    let bearer_token = header_value.to_str().unwrap();

                    let token = bearer_token.replace("Bearer ", "");

                    let token_msg = TokenMsg { token, ip };

                    //进行token认证
                    match GLOB_JOT.validation_token(&token_msg) {
                        Ok(data) => {
                            /*判断用户权限是否存在*/
                            if GLOBAL_YAML_CONFIG
                                .authority
                                .auth
                                .clone()
                                .unwrap()
                                .contains(&data.auth)
                            {
                                /*将用户信息传下去*/
                                req.extensions_mut().insert(data);

                                let fut = self.service.call(req);
                                Box::pin(async move {
                                    let res = fut.await?;
                                    Ok(res)
                                })
                            } else {
                                Box::pin(async {
                                    Err(ErrorUnauthorized("Permission does not exist"))
                                })
                            }
                        }
                        Err(err) => {
                            return match err {
                                AuthError::VerifyError => Box::pin(async {
                                    Err(ErrorUnauthorized("Authentication failure"))
                                }),
                                AuthError::ExpirationError => {
                                    Box::pin(async { Err(ErrorUnauthorized("Token Expired")) })
                                }
                            };
                        }
                    }
                }
            };
        } else {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        };
    }
}

fn is_need_verification(path: &str) -> bool {
    !(path == "/api/login"
        || path == "/api/register"
        || path == "/api/sendmail"
        || path == "/api/forget"
        || path == "/api/email-login")
}
