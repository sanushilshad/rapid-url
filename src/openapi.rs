
use utoipa::openapi::Object;
use utoipa::OpenApi;
use utoipauto::utoipauto;
use crate::schemas::CreateUrlResponseData;
#[utoipauto]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "ONDC Rapid URL REST API", description = "ONDC Rapid URLAPI Endpoints")
    ),
)]

pub struct ApiDoc {}
