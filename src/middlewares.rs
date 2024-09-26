use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{http, web, Error, HttpMessage};
use futures::future::LocalBoxFuture;
use uuid::Uuid;
use std::future::{ready, Ready};
use std::rc::Rc;
use crate::errors::GenericError;
use crate::schemas::SecretSetting;
use crate::utils::decode_token;


pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<actix_web::body::BoxBody>, Error = Error>
        + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        let jwt_secret = &req
            .app_data::<web::Data<SecretSetting>>()
            .unwrap()
            .jwt
            .secret;

        if token.is_none() {
            let error_message = "Authorization header is missing".to_string();
            let (request, _pl) = req.into_parts();
            let json_error = GenericError::ValidationError(error_message);
            return Box::pin(async { Ok(ServiceResponse::from_err(json_error, request)) });
        }

        let uuid = match decode_token(token.unwrap(), jwt_secret) {
            Ok(id) => id,
            Err(e) => {
                return Box::pin(async move {
                    let (request, _pl) = req.into_parts();
                    Ok(ServiceResponse::from_err(
                        GenericError::InvalidJWT(e.to_string()),
                        request,
                    ))
                });
            }
        };
        req.extensions_mut().insert::<Uuid>(uuid);
        
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

/// Middleware factory for requiring authentication.
pub struct RequireAuth;

impl<S> Transform<S, ServiceRequest> for RequireAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<actix_web::body::BoxBody>, Error = Error>
        + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}
