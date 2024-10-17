
use std::{fs, io};

use chrono::{Duration, Utc};
use config::{ConfigError, Environment};
use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;

use crate::{errors::CustomJWTTokenError, schemas::{DatabaseSettings, JWTClaims, Settings}};
use secrecy::{ExposeSecret, SecretString};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use jsonwebtoken::{
    decode, encode, Algorithm as JWTAlgorithm, DecodingKey, EncodingKey, Header, Validation,
};

#[tracing::instrument(name = "generate_short_url", skip())]
pub fn generate_short_url() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6) 
        .map(char::from)
        .collect()
}



#[tracing::instrument(name = "Decode JWT token")]
pub fn decode_token<T: Into<String> + std::fmt::Debug>(
    token: T,
    secret: &SecretString,
) -> Result<Uuid, CustomJWTTokenError> {
    let decoding_key = DecodingKey::from_secret(secret.expose_secret().as_bytes());
    let decoded = decode::<JWTClaims>(
        &token.into(),
        &decoding_key,
        &Validation::new(JWTAlgorithm::HS256),
    );
    match decoded {
        Ok(token) => Ok(token.claims.sub),
        Err(e) => {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    Err(CustomJWTTokenError::Expired)
                }
                _ => Err(CustomJWTTokenError::Invalid("Invalid Token".to_string())),
            }
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}



#[tracing::instrument(name = "insert_url", skip(pool))]
pub async fn insert_url(
    pool: &PgPool,
    original_url: &str,
    short_url: &str,
    user_id: &Uuid,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
        INSERT INTO short_url (original_url, short_url, created_on, user_id) 
        VALUES ($1, $2, $3, $4)
        "#,
        original_url,
        short_url,
        Utc::now(),
        user_id
    )
    .execute(pool)
    .await
    .map_err(|e| anyhow::anyhow!("Database error: {:?}", e))?;

    Ok(())
}

pub async fn get_original_url(pool: &PgPool, short_url: &str) -> sqlx::Result<Option<String>> {
    let result = sqlx::query_scalar!(
        "SELECT original_url FROM short_url WHERE short_url = $1",
        short_url
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(result)
}


pub fn get_configuration() -> Result<Settings, ConfigError> {
    let builder = config::Config::builder()
        .add_source(Environment::default().separator("__"))
        .add_source(
            Environment::with_prefix("LIST")
                .try_parsing(true)
                .separator("__")
                .keep_prefix(false)
                .list_separator(","),
        )
        .build()?;
    builder.try_deserialize::<Settings>()
}

#[tracing::instrument(name = "Execute Queries")]
async fn execute_query(path: &str, pool: &PgPool) -> io::Result<()> {
    let migration_files = fs::read_dir(path)?;
    for migration_file in migration_files {
        let migration_file = migration_file?;
        let migration_path = migration_file.path();
        let migration_sql = fs::read_to_string(&migration_path)?;
        let statements: String = migration_sql.replace('\n', "");
        let new_statement: Vec<&str> = statements
            .split(';')
            .filter(|s| !s.trim().is_empty() & !s.starts_with("--"))
            .collect();
        for statement in new_statement {
            if let Err(err) = sqlx::query(statement).execute(pool).await {
                eprintln!("Error executing statement {:?}: {} ", statement, err);
            } else {
                eprintln!("Migration applied: {:?}", statement);
            }
        }

        eprintln!("Migration applied: {:?}", migration_path);
    }

    Ok(())
}



#[tracing::instrument(name = "Create Database")]
pub async fn create_database(config: &DatabaseSettings) {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    let db_count: Result<Option<i64>, sqlx::Error> =
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM pg_database WHERE datname = $1")
            .bind(&config.name)
            .fetch_optional(&mut connection)
            .await;

    match db_count {
        Ok(Some(count)) => {
            if count > 0 {
                tracing::info!("Database {} already exists.", &config.name);
            } else {
                connection
                    .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
                    .await
                    .expect("Failed to create database.");
                eprintln!("Database created.");
            }
        }
        Ok(_) => eprintln!("No rows found."),
        Err(err) => eprintln!("Error: {}", err),
    }

    let test_db_count: Result<Option<i64>, sqlx::Error> =
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM pg_database WHERE datname = $1")
            .bind(&config.test_name)
            .fetch_optional(&mut connection)
            .await;

    match test_db_count {
        Ok(Some(count)) => {
            if count > 0 {
                eprintln!("Test database {} already exists.", &config.test_name);
            } else {
                connection
                    .execute(format!(r#"CREATE DATABASE "{}";"#, config.test_name).as_str())
                    .await
                    .expect("Failed to create test database.");
                eprintln!("Test database {} created.", &config.test_name);
            }
        }
        Ok(_) => eprintln!("No rows found for the test database check."),
        Err(err) => eprintln!("Error checking test database existence: {}", err),
    }
}


#[tracing::instrument(name = "Confiure Database")]
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    create_database(config).await;
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    let test_connection_pool = PgPool::connect_with(config.test_with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let _ = execute_query("./migrations", &connection_pool).await;
    let _ = execute_query("./migrations", &test_connection_pool).await;
    connection_pool
}



#[tracing::instrument(name = "Generate JWT token for user")]
pub fn generate_jwt_token_for_user(
    user_id: Uuid,
    expiry_time: i64,
    secret: &SecretString,
) -> Result<SecretString, anyhow::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(expiry_time))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims: JWTClaims = JWTClaims {
        sub: user_id,
        exp: expiration,
    };
    let header = Header::new(JWTAlgorithm::HS256);
    let encoding_key = EncodingKey::from_secret(secret.expose_secret().as_bytes());
    let token: String = encode(&header, &claims, &encoding_key).expect("Failed to generate token");
    Ok(SecretString::new(token.into()))
}


pub async fn get_user_id(pool: &PgPool, username: &str) ->  Result<Option<Uuid>, anyhow::Error>{
    let result = sqlx::query_scalar!(
        "SELECT id FROM user_account WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;
    Ok(result)
}


#[tracing::instrument(name = "delete_short_urls", skip(pool))]
pub async fn delete_short_urls(
    pool: &PgPool,
) -> Result<(), anyhow::Error> {
    let _ = sqlx::query("DELETE FROM short_urls")
    .execute(pool)
    .await;
    Ok(())
}