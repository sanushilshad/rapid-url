
use crate::handlers::{create_short_url, redirect_short_url};
use crate::middlewares::RequireAuth;
use crate::openapi::ApiDoc;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn routes(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    cfg

        .service(
            web::resource("/shorten").route(web::post().to(create_short_url).wrap(RequireAuth))
        )
        .service(  web::resource("/{short_url}").route(web::get().to(redirect_short_url)))
        .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone()));
}
