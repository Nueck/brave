use std::future::{ready, Ready};

use actix_web::http::header;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error, HttpResponse,
};
use brave_config::init::InitStatus;
use brave_config::interface::Interface;
use futures_util::future::LocalBoxFuture;

pub struct HeadCheck;

impl<S, B> Transform<S, ServiceRequest> for HeadCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = HeadCheckMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(HeadCheckMiddleware { service }))
    }
}

pub struct HeadCheckMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for HeadCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        //添加head的认证
        /*加可以通过的get请求url*/
        let from_browser = request.method() == http::Method::GET;

        if from_browser {
            let (request, _pl) = request.into_parts();
            let response = redirect_index().map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        } else {
            let res = self.service.call(request);
            Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
        }
    }
}

pub fn redirect_index() -> HttpResponse {
    /*根据初始化状态*/
    if InitStatus::global().is_init {
        let home = Interface::redirect_home();
        HttpResponse::Found()
            .append_header((header::LOCATION, home))
            .finish()
    } else {
        /*重定向到初始化*/
        let init_add = Interface::redirect_init_address();
        HttpResponse::Found()
            .append_header((header::LOCATION, init_add))
            .finish()
    }
}
