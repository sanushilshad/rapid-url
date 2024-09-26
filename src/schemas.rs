use futures::future::LocalBoxFuture;
use secrecy::{ ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::ConnectOptions;
use utoipa::{openapi::Object, ToSchema};
use sqlx::postgres::PgConnectOptions;
use uuid::Uuid;

use crate::errors::GenericError;
use actix_web::{ web, FromRequest, HttpRequest};

use actix_http::Payload;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CreateUrlRequest {
    pub original_url: String,
    pub expiry_date: Option<DateTime<Utc>>,
    
}

impl FromRequest for CreateUrlRequest {
    type Error = GenericError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let fut = web::Json::<Self>::from_request(req, payload);

        Box::pin(async move {
            match fut.await {
                Ok(json) => Ok(json.into_inner()),
                Err(e) => Err(GenericError::ValidationError(e.to_string())),
            }
        })
    }
}


#[derive(Debug, Serialize, ToSchema)]
pub struct CreateUrlResponseData {
    pub short_url: String,

}


#[derive(Debug, Deserialize, Serialize)]
pub struct JWTClaims {
    pub sub: Uuid,
    pub exp: usize,
}


#[derive(Serialize, Debug, ToSchema)]
#[aliases(EmptyGenericResponse = GenericResponse<Object>,  CreateUrlResponse = GenericResponse<CreateUrlResponseData>)]
pub struct GenericResponse<D> {
    pub status: bool,
    pub customer_message: String,
    pub code: String,
    pub data: Option<D>,
}

impl<D> GenericResponse<D> {
    pub fn success(message: &str, data: Option<D>) -> Self {
        Self {
            status: true,
            customer_message: String::from(message),
            code: String::from("200"),
            data,
        }
    }

    pub fn error(message: &str, code: &str, data: Option<D>) -> Self {
        Self {
            status: false,
            customer_message: String::from(message),
            code: String::from(code),
            data,
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub name: String,
    pub test_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
}


impl DatabaseSettings {
    // Renamed from `connection_string_without_db`
    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
    }
    // Renamed from `connection_string`
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.name) 
            .log_statements(tracing::log::LevelFilter::Trace)
    }

    pub fn test_with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.test_name)
            .log_statements(tracing::log::LevelFilter::Trace)
    }
}
#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub workers: usize,
}



#[derive(Debug, Deserialize, Clone)]
pub struct JWT {
    pub secret: Secret<String>,
    pub expiry: i64,
}



#[derive(Debug, Deserialize, Clone)]
pub struct SecretSetting {
    pub jwt: JWT,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub secret: SecretSetting,
}



#[derive(Serialize, Deserialize, Debug, sqlx::Type, ToSchema)]
#[sqlx(type_name = "data_source", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DataSource {
    PlaceOrder,
    TradeIndia,
    Rapidor,
}