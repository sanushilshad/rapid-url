
use utoipa::OpenApi;
use utoipauto::utoipauto;
#[utoipauto]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "ONDC Rapid URL REST API", description = "ONDC Rapid URL API Endpoints")
    ),
    info(
        title = "ONDC Rapid URL API",
        description = "ONDC Rapid URL API Endpoints",
        version = "1.0.0",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
)]

pub struct ApiDoc {}
