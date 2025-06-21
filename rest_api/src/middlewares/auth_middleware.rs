use crate::errors::ApiError;
use crate::utils::jwt::decode_jwt;
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareImpl<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareImpl {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareImpl<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareImpl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Cek cookie auth_token
            let token = req.cookie("auth_token").map(|c| c.value().to_string());

            let token = match token {
                Some(t) => t,
                None => return Err(ApiError::Unauthorized("Token tidak ditemukan".into()).into()),
            };

            // Validasi token JWT
            let _decoded = decode_jwt(&token)
                .map_err(|_| ApiError::Unauthorized("Token tidak valid atau expired".into()))?;

            // Cek CSRF token jika method bukan GET
            if req.method() != actix_web::http::Method::GET {
                let csrf_cookie = req.cookie("csrf_token").map(|c| c.value().to_string());
                let csrf_header = req
                    .headers()
                    .get("x-csrf-token")
                    .and_then(|v| v.to_str().ok());

                if csrf_cookie.is_none() || csrf_header != csrf_cookie.as_deref() {
                    return Err(ApiError::Forbidden("CSRF token tidak cocok".into()).into());
                }
            }

            service.call(req).await
        })
    }
}
