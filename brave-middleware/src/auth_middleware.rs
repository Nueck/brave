use std::future::{ready, Ready};
use std::task::{Context, Poll};

use crate::GLOB_JOT;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use brave_utils::error::AuthError;
use brave_utils::jwt::jwt::TokenMsg;
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

    //其中的左右用stackoverflow的大佬解释
    //I'm kind of late to the party but the best way to do this from within
    //Actix middleware is using futures::future::Either.
    //You can see how it's used here: https://github.com/actix/examples/blob/master/middleware/middleware/src/redirect.rs.
    //The left hand side of Either will be a Future which passes the response to the next stage
    //in the chain. The right hand side will be a response (usually HttpResponse)
    //if you wish to return the response early.

    fn call(&self, req: ServiceRequest) -> Self::Future {
        //获取ip
        let addr = req.peer_addr().unwrap();
        let ip = addr.ip().to_string();

        return if req.path() == "/api/login" {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            //首先判断有没有认证
            return match req.headers().get(header::AUTHORIZATION) {
                None => {
                    log::info!("- {} There is no authentication token", ip);
                    const MESSAGE: &str = "There is no authentication token";
                    let json =
                        serde_json::json!({"status": "no authentication", "message": MESSAGE});
                    Box::pin(async { Err(ErrorUnauthorized(json)) })
                }
                Some(header_value) => {
                    let bearer_token = header_value.to_str().unwrap();

                    let token = bearer_token.replace("Bearer ", "");

                    let token_msg = TokenMsg { token, ip };

                    //进行token认证
                    match GLOB_JOT.validation_token(&token_msg) {
                        Ok(data) => {
                            req.extensions_mut().insert(data);

                            let fut = self.service.call(req);
                            Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            })
                        }
                        Err(err) => {
                            return match err {
                                AuthError::VerifyError => {
                                    const MESSAGE: &str = "Authentication failure";
                                    let json = serde_json::json!({"status": "failure", "message": MESSAGE});
                                    Box::pin(async { Err(ErrorUnauthorized(json)) })
                                }
                                AuthError::ExpirationError => {
                                    const MESSAGE: &str = "Login expired";
                                    let json = serde_json::json!({"status": "expired", "message": MESSAGE});
                                    Box::pin(async { Err(ErrorUnauthorized(json)) })
                                }
                            };
                        }
                    }
                }
            };
        };
    }
}
