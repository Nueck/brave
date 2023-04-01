use std::future::{ready, Ready};
use std::task::{Context, Poll};

use actix_web::error::{ErrorNetworkAuthenticationRequired, ErrorUnauthorized};
use actix_web::http::header;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use brave_config::GLOBAL_CONFIG;
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

        if is_need_verification(req.path()) {
            //首先判断有没有认证
            match req.headers().get(header::AUTHORIZATION) {
                None => Box::pin(async {
                    const MSG: &str = "No Authentication Header";
                    let json = serde_json::json!({"state": "error", "message": MSG });
                    Err(ErrorNetworkAuthenticationRequired(json))
                }),
                Some(header_value) => {
                    let bearer_token = header_value.to_str().unwrap();

                    let token = bearer_token.replace("Bearer ", "");

                    let token_msg = TokenMsg { token, ip };

                    //进行token认证
                    match GLOB_JOT.validation_token(&token_msg) {
                        Ok(data) => {
                            /*判断用户权限是否存在*/
                            if GLOBAL_CONFIG
                                .authority
                                .auth
                                .clone()
                                .unwrap()
                                .contains(&data.auth)
                            {
                                if data.refresh {
                                    /*判断是否刷新token*/
                                    if refresh_api(req.path()) {
                                        req.extensions_mut().insert(data);

                                        let fut = self.service.call(req);
                                        Box::pin(async move {
                                            let res = fut.await?;
                                            Ok(res)
                                        })
                                    } else {
                                        Box::pin(async {
                                            const MSG: &str =
                                                "The refresh token cannot be used on this api";
                                            let json = serde_json::json!({"state": "error", "message": MSG });
                                            Err(ErrorUnauthorized(json))
                                        })
                                    }
                                } else {
                                    /*将用户信息传下去*/
                                    req.extensions_mut().insert(data);

                                    let fut = self.service.call(req);
                                    Box::pin(async move {
                                        let res = fut.await?;
                                        Ok(res)
                                    })
                                }
                            } else {
                                Box::pin(async {
                                    const MSG: &str = "Permission does not exist";
                                    let json =
                                        serde_json::json!({"state": "error", "message": MSG });
                                    Err(ErrorUnauthorized(json))
                                })
                            }
                        }
                        Err(err) => match err {
                            AuthError::VerifyError => Box::pin(async {
                                const MSG: &str = "Authentication failure";
                                let json = serde_json::json!({"state": "error", "message": MSG });
                                Err(ErrorUnauthorized(json))
                            }),
                            AuthError::ExpirationError => Box::pin(async {
                                const MSG: &str = "Token Expired";
                                let json = serde_json::json!({"state": "error", "message": MSG });
                                Err(ErrorUnauthorized(json))
                            }),
                        },
                    }
                }
            }
        } else {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        }
    }
}

fn is_need_verification(path: &str) -> bool {
    !(path == format!("/{}/init", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/init-status", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/login", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/register", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/sendmail", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/forget", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/{}/email-login", GLOBAL_CONFIG.interface.api_scope)
        || path == format!("/api/upload/img"))
}

fn refresh_api(path: &str) -> bool {
    path == format!("/{}/updateToken", GLOBAL_CONFIG.interface.api_scope)
}
