
use crate::handlers::{create_short_url, redirect_short_url};
use crate::middlewares::RequireAuth;
use crate::openapi::ApiDoc;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn routes(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    cfg.route("/{short_url}", web::get().to(redirect_short_url))
        .route("/shorten", web::post().to(create_short_url).wrap(RequireAuth))
        .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone()));
}
