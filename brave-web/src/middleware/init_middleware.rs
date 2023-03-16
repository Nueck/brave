use std::future::{ready, Ready};
use std::task::{Context, Poll};

use crate::config::InitStatus;
use actix_web::error::ErrorUnauthorized;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

pub struct InitAuth;

impl<S, B> Transform<S, ServiceRequest> for InitAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = InitAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InitAuthMiddleware { service }))
    }
}

pub struct InitAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for InitAuthMiddleware<S>
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
        if is_need_verification(req.path()) {
            /*判断是否初始化,如果没有初始化其他接口无法调用，只能访问博客*/
            if InitStatus::global().is_init {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            } else {
                Box::pin(async { Err(ErrorUnauthorized("Need to initialize")) })
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
    !(path == "/api/init" || path == "/api/init-status")
}
