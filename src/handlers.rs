use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use crate::{errors::GenericError, schemas::ApplicationSettings, utils::{generate_short_url, get_original_url, insert_url}};
use sqlx::PgPool;
use uuid::Uuid;

use crate::schemas::{CreateUrlRequest, CreateUrlResponseData, GenericResponse};



#[utoipa::path(
    post,
    path = "/shorten",
    tag = "Create short URL",
    request_body(content = CreateUrlRequest, description = "Request Body"),
    responses(
        (status=200, description= "Create short URL", body= GenericResponse<CreateUrlResponseData>),
    ),
    params(
        ("Authorization" = String, Header, description = "JWT token"),
    )
)]
#[tracing::instrument(name = "create_short_url", skip(pool))]
pub async fn create_short_url(
    pool: web::Data<PgPool>,
    req: CreateUrlRequest,
    request: HttpRequest, 
    application: web::Data<ApplicationSettings>,
) -> Result<web::Json<GenericResponse<CreateUrlResponseData>>, GenericError>{
    let user_id = match request.extensions().get::<Uuid>() {
        Some(uuid) => *uuid, 
        None => return Err(GenericError::ValidationError("User ID not found".to_string())),
    };
    
    let short_url = generate_short_url();
    match insert_url(&pool, &req.original_url, &short_url, &user_id).await {
        Ok(_) => Ok(web::Json(GenericResponse::success(
            "Successfully created short url",
            Some(CreateUrlResponseData {
                short_url: format!("https://{}/{}", &application.domain, &short_url),
            }),
        ))),
        Err(_) =>  Err(GenericError::ValidationError("Internal Server Error".to_string())),
    }
}



#[utoipa::path(
    post,
    path = "/{short_url}",
    tag = "Redirect short URL",
    responses(
        (status=200, description= "Redirect short URL"),
    )
)]
#[tracing::instrument(name = "redirect_short_url", skip(pool))]
pub async fn redirect_short_url(
    pool: web::Data<PgPool>,
    short_url: web::Path<String>,
) -> impl Responder {
    match get_original_url(&pool, &short_url).await {
        Ok(Some(original_url)) => HttpResponse::Found()
            .append_header(("Location", original_url))
            .finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}

