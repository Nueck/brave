use std::future::{ready, Ready};
use std::task::{Context, Poll};

use crate::GLOB_JOT;
use actix_web::http::header;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
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
    type Response = ServiceResponse<EitherBody<B>>;
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
    type Response = ServiceResponse<EitherBody<B>>;
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
            let res = self.service.call(req);
            Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
        } else {
            //首先判断有没有认证
            match req.headers().get(header::AUTHORIZATION) {
                None => {
                    log::info!("- {} There is no authentication token", ip);
                    let (request, _pl) = req.into_parts();
                    const MESSAGE: &str = "There is no authentication token";
                    let response = HttpResponse::Unauthorized()
                        .json(
                            serde_json::json!({"status": "no authentication", "message": MESSAGE}),
                        )
                        .map_into_right_body();
                    Box::pin(async { Ok(ServiceResponse::new(request, response)) })
                }
                Some(header_value) => {
                    let bearer_token = header_value.to_str().unwrap();

                    let token = bearer_token.replace("Bearer ", "");

                    let token_msg = TokenMsg { token, ip };

                    //进行token认证
                    return match GLOB_JOT.validation_token(&token_msg) {
                        Ok(_) => {
                            let res = self.service.call(req);

                            Box::pin(
                                async move { res.await.map(ServiceResponse::map_into_left_body) },
                            )
                        }
                        Err(err) => {
                            return match err {
                                AuthError::VerifyError => {
                                    let (request, _pl) = req.into_parts();
                                    const MESSAGE: &str = "Authentication failure";
                                    let response = HttpResponse::Unauthorized()
                                        .json(serde_json::json!({"status": "failure", "message": MESSAGE}))
                                        .map_into_right_body();
                                    Box::pin(async { Ok(ServiceResponse::new(request, response)) })
                                }
                                AuthError::ExpirationError => {
                                    let (request, _pl) = req.into_parts();
                                    const MESSAGE: &str = "Login expired";
                                    let response = HttpResponse::Unauthorized()
                                        .json(serde_json::json!({"status": "expired", "message": MESSAGE}))
                                        .map_into_right_body();
                                    Box::pin(async { Ok(ServiceResponse::new(request, response)) })
                                }
                            };
                        }
                    };
                }
            }
        };
    }
}
